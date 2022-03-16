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
    /*Create Pathfinder*/
    let pathfinder_local = get_pathfinder(tilemap_struct);
    /*******************/

    let mut selected_entity: u32 = 0;
    //Add Engineer Sprite
    for _i in 0..8 {
        let position = vec2(rand::gen_range::<f32>(1.0, 10.0) as f32, rand::gen_range::<f32>(1.0, 10.0) as f32);
        let uuid: u32 = rand::rand();
        let engy_sprite: SpriteID = SpriteID::Engineer(Engineer {
            texture: engineer_anim,
            animated_sprite: AnimatedSprite::new(
                64,
                64,
                &[Animation {
                    name: "N".to_string(),
                    row: 0,
                    frames: 17, //18 frames including original frame which we want to skip.
                    fps: 30,
                },Animation {
                    name: "NW".to_string(),
                    row: 1,
                    frames: 17, //18 frames including original frame which we want to skip.
                    fps: 30,
                },Animation {
                    name: "W".to_string(),
                    row: 2,
                    frames: 17, //18 frames including original frame which we want to skip.
                    fps: 30,
                },Animation {
                    name: "SW".to_string(),
                    row: 3,
                    frames: 17, //18 frames including original frame which we want to skip.
                    fps: 30,
                },Animation {
                    name: "S".to_string(),
                    row: 4,
                    frames: 17, //18 frames including original frame which we want to skip.
                    fps: 30,
                },Animation {
                    name: "SE".to_string(),
                    row: 5,
                    frames: 17, //18 frames including original frame which we want to skip.
                    fps: 30,
                },Animation {
                    name: "E".to_string(),
                    row: 6,
                    frames: 17, //18 frames including original frame which we want to skip.
                    fps: 30,
                },Animation {
                    name: "NE".to_string(),
                    row: 7,
                    frames: 17, //18 frames including original frame which we want to skip.
                    fps: 30,
                }],
                true,
            ),
            x: grid_to_world_coords(position.floor()).x,
            y: grid_to_world_coords(position.floor()).y,
            current_path: Vec::new(),
            previous_position: TilePosition{x:position.x.floor() as i32,y:position.y.floor() as i32},
            uuid: uuid,
        });
        sprite_map_store.insert(uuid,engy_sprite);
    }

    play_sound(
        background_music,
        PlaySoundParams {
            looped: true,
            volume: 0.1,
        },
    );
    let mut last_time_recorded: f64 = get_time();
    let mut render_list: Vec<u32> = Vec::new();
    for (uuid, _sprite) in sprite_map_store.iter()
    {
        render_list.push(*uuid);
    }

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
        render_list.sort_by(|a, b| {let ordering = sprite_map_store.get(a).unwrap().get_zindex().cmp(&sprite_map_store.get(b).unwrap().get_zindex()); if ordering==std::cmp::Ordering::Equal {
            let mut a_val = 0;
            let mut b_val = 0;
           match
            sprite_map_store.get(a).unwrap()
            {
                SpriteID::Tile(sprite_move_request) => a_val=1,
                Default => b_val=b_val,
            }
            match
             sprite_map_store.get(b).unwrap()
             {
                 SpriteID::Tile(sprite_move_request) => b_val=1,
                 Default => b_val=b_val,
             }
             return a_val.cmp(&b_val);
        }
        else
        {
            return ordering;
        }
    });
        /********************************/

        /*Get current game clock time*/
        let current_time = get_time();

        let mut selected_texture_location: Vec2 = vec2(-1., -1.);

        let (mouse_x, mouse_y) = mouse_position();
        let mut just_selected: bool = false;
        let mut world_vec: Vec2 = vec2(-1., -1.);
        let mut grid_coords: Vec2 = vec2(-1., -1.);
        let mut just_clicked: bool = false;
        if is_mouse_button_released(MouseButton::Left) {
            world_vec = camera.screen_to_world(vec2(mouse_x, mouse_y));
            grid_coords = world_to_grid_coords(world_vec);
            just_clicked = true;

            println!("World X: {}", world_vec.x);
            println!("World Y: {}", world_vec.y);
            println!("Grid X: {}", world_to_grid_coords(world_vec).x);
            println!("Grid Y: {}", world_to_grid_coords(world_vec).y);
        }

        for uuid in &render_list {
            let sprite = sprite_map_store.get_mut(&uuid).unwrap();
            match sprite {
                SpriteID::Engineer(engineer_entity) => {
                    engineer_entity.update_view(current_time - last_time_recorded);
                    if just_clicked {
                        if engineer_entity.is_within_bounds(world_vec) {
                            just_selected = true;
                            selected_entity = engineer_entity.uuid;
                        }
                    }
                    if selected_entity == engineer_entity.uuid {
                        selected_texture_location =
                            vec2(engineer_entity.x, engineer_entity.y - 5.);
                    }
                }
                SpriteID::Tile(_tile_entity) => {}
            };
            sprite.draw(); //Draw all sprites in Sprite List
        }

        if selected_entity != 0 {
            draw_texture_ex(
                selected_texture,
                selected_texture_location.x,
                selected_texture_location.y,
                color::WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(64.0, 64.0)),
                    source: Some(Rect::new(0., 0., 64., 64.)),
                    ..Default::default()
                },
            );
        }
        last_time_recorded = current_time;

        if !just_selected && (grid_coords.x > 0.) {
            match &mut sprite_map_store.get_mut(&selected_entity).unwrap() {
                SpriteID::Engineer(engineer_entity) => {
                    let mut path = pathfinder_local.find_path(
                        TilePosition {
                            x: engineer_entity.get_tile_pos().x as i32,
                            y: engineer_entity.get_tile_pos().y as i32,
                        },
                        TilePosition {
                            x: (grid_coords.x - 1.0) as i32,
                            y: (grid_coords.y - 0.5) as i32,
                        },
                    );

                    engineer_entity.update_path(std::mem::take(&mut path))
                }

                SpriteID::Tile(_tile) => {}
            }
        }
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
