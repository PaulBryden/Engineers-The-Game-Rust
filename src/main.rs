use crate::model::gamestate::GameState;
use crate::model::gamemanager::GameManager;
use include_dir::include_dir;
use include_dir::Dir;
use macroquad::prelude::*;
mod pathfinding {
    pub mod pathfinder;
}
use macroquad::{
    audio::{load_sound_from_bytes, play_sound, PlaySoundParams},
    color,
    experimental::animation::{AnimatedSprite, Animation},
};
mod sprites {
    pub mod engineersprite;
    pub mod sprite;
    pub mod tilesprite;
}
mod model {
    pub mod requests;
    pub mod gamemanager;
    pub mod gamestate;
}
pub mod tiledmap;
use pathfinding::pathfinder::{Pathfinder, TilePosition};
use sprites::engineersprite::Engineer;
use sprites::sprite::{grid_to_world_coords, world_to_grid_coords, Sprite, SpriteID};
use sprites::tilesprite::TileSprite;
use model::requests::*;
#[macroquad::main("engineers")]
async fn main() {
    static ASSETS_DIR: Dir = include_dir!("assets");
    static MUSIC_DIR: Dir = include_dir!("assets/music");
    /*Load Assets*/
    let tileset_png = ASSETS_DIR.get_file("tileset.png").unwrap();
    let selected_png = ASSETS_DIR.get_file("selected.png").unwrap();
    let spritesheet_rock_png = ASSETS_DIR.get_file("spritesheet_rock.png").unwrap();
    let background_music_ogg = MUSIC_DIR.get_file("background_music.ogg").unwrap();
    let tileset = Texture2D::from_file_with_format(tileset_png.contents(), None);
    let selected_texture = Texture2D::from_file_with_format(selected_png.contents(), None);
    let engineer_anim = Texture2D::from_file_with_format(spritesheet_rock_png.contents(), None);
    let background_music = load_sound_from_bytes(background_music_ogg.contents())
        .await
        .unwrap();
    /****************************/

    /*Load JSON Tilemap*/
    let lib_rs = ASSETS_DIR.get_file("tiledmap.json").unwrap();
    let body = lib_rs.contents_utf8().unwrap();
    let tilemap_struct: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
    /****************************/

    /*Seed random number generator*/
    rand::srand((get_time() * 99999.99) as u64);
    /**************************/

    /*Generate Tiled Sprite List*/
    let mut sprite_map_store: std::collections::HashMap<u32,SpriteID> = get_tilemap_spritelist(tileset, &tilemap_struct);
    let mut render_list: Vec<u32> = Vec::new();
    for (uuid, _sprite) in sprite_map_store.iter()
    {
        render_list.push(*uuid);
    }
    let game_state: GameState = GameState{sprite_map:sprite_map_store, sprite_uuid_list:render_list, selected_entity: 0};
    let pathfinder_local = get_pathfinder(tilemap_struct);
    let mut game_manager: GameManager = GameManager{requests: RequestQueue::default(), game_state_history: std::collections::HashMap::new(), current_game_state: game_state, last_tick: 0, pathfinder: pathfinder_local};
    /*Create Pathfinder*/
    /*******************/

    let mut selected_entity: u32 = 0;
    //Add Engineer Sprite
    for _i in 0..8 {
        let position = vec2(rand::gen_range::<f32>(1.0, 10.0) as f32, rand::gen_range::<f32>(1.0, 10.0) as f32);
        let uuid: u32 = rand::rand();
        let request: Request = Request::SpriteCreate(SpriteCreateRequest {tick:40 - position.x as u32, sprite_uuid: uuid, position: TilePosition{x: position.x as i32,y: position.y as i32}, sprite_type: SpriteType::Engineer});
        game_manager.addLocalRequest(request);
    }

    play_sound(
        background_music,
        PlaySoundParams {
            looped: true,
            volume: 0.1,
        },
    );
    let mut last_tick_time: f64 = get_time();
    let mut tick_count =0;
    game_manager.current_game_state.process_tick(tick_count);
    loop {
        clear_background(BLACK);
        let camera = Camera2D {
            zoom: vec2(
                1. / macroquad::window::screen_width() * 2.,
                -1. / macroquad::window::screen_height() * 2.,
            ),
            target: vec2(0.0, 100.0),
            ..Default::default()
        };
        set_camera(&camera);
        //Z-Index Sorting for render ordering
        
        /********************************/

        /*Get current game clock time*/
        let current_time = get_time();

        let (mouse_x, mouse_y) = mouse_position();
        let mut world_vec: Vec2 = vec2(-1., -1.);
        let mut grid_coords: Vec2 = vec2(-1., -1.);
        if is_mouse_button_released(MouseButton::Left) {
            world_vec = camera.screen_to_world(vec2(mouse_x, mouse_y));
            grid_coords = world_to_grid_coords(world_vec);
            game_manager.mouse_clicked(world_vec);
            println!("World X: {}", world_vec.x);
            println!("World Y: {}", world_vec.y);
            println!("Grid X: {}", world_to_grid_coords(world_vec).x);
            println!("Grid Y: {}", world_to_grid_coords(world_vec).y);
        }


        while(current_time-last_tick_time>=0.05)
        {
            tick_count=tick_count+1;
            game_manager.process_tick(tick_count);
            last_tick_time=last_tick_time+0.05;
        }
        game_manager.render();

        next_frame().await;
    }
}

pub fn get_pathfinder(tilemap: tiledmap::TiledMap) -> Pathfinder {
    Pathfinder::new(tilemap)
}

pub fn get_tilemap_spritelist(tileset: Texture2D, tilemap: &tiledmap::TiledMap) -> std::collections::HashMap<u32,SpriteID> {
    let mut sprite_store: std::collections::HashMap<u32,SpriteID> = std::collections::HashMap::new();
    for layer_num in 0 as usize..tilemap.layers.len() as usize {
        for y in 0 as u32..tilemap.height as u32 {
            for x in 0 as u32..tilemap.width as u32 {
                let uuid: u32 = rand::rand();
                let tile_sprite: SpriteID = SpriteID::Tile(TileSprite {
                    layer: layer_num as u32,
                    x: x,
                    y: y,
                    texture: tileset,
                    frame_number: tilemap.layers[layer_num].data
                        [(y * tilemap.layers[layer_num].width as u32 + x) as usize]
                        as u32
                        - 1,
                    width: 64.0,
                    height: 32.0,
                    uuid: uuid,
                });
                sprite_store.insert(uuid, tile_sprite);
            }
        }
    }
    sprite_store
}

pub fn aspect_ratio() -> f32 {
    macroquad::window::screen_width() / macroquad::window::screen_height()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_collision_tile() {
        static ASSETS_DIR: Dir = include_dir!("assets");
        let lib_rs = ASSETS_DIR.get_file("tiledmap.json").unwrap();
        let body = lib_rs.contents_utf8().unwrap();
        let map_cast: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
        let pathfinder: Pathfinder = Pathfinder::new(map_cast);

        assert_eq!(pathfinder.tile_is_walkable(0, 0), false);
    }

    #[test]
    fn test_non_collision_tile() {
        static ASSETS_DIR: Dir = include_dir!("assets");
        let lib_rs = ASSETS_DIR.get_file("tiledmap.json").unwrap();
        let body = lib_rs.contents_utf8().unwrap();
        let map_cast: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
        let pathfinder: Pathfinder = Pathfinder::new(map_cast);

        assert_eq!(pathfinder.tile_is_walkable(1, 2), true);
    }

    #[test]
    fn test_collision_tile_2() {
        static ASSETS_DIR: Dir = include_dir!("assets");
        let lib_rs = ASSETS_DIR.get_file("tiledmap.json").unwrap();
        let body = lib_rs.contents_utf8().unwrap();
        let map_cast: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
        let pathfinder: Pathfinder = Pathfinder::new(map_cast);

        assert_eq!(pathfinder.tile_is_walkable(20, 1), false);
    }
}
