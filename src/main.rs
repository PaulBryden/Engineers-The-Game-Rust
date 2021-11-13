use include_dir::include_dir;
use include_dir::Dir;
use macroquad::prelude::*;
mod pathfinding
{
    pub mod pathfinder;
}
use macroquad::{
    audio::{play_sound,PlaySoundParams, 
    load_sound_from_bytes},
    experimental::{
        animation::{AnimatedSprite, Animation},
    },
};
 mod sprites{
    pub mod sprite;
    pub mod tilesprite;
    pub mod engineersprite;
}
pub mod tiledmap;
use sprites::sprite::{Sprite, grid_to_world_coords,world_to_grid_coords};
use sprites::tilesprite::TileSprite;
use sprites::engineersprite::Engineer;
use pathfinding::pathfinder::{Pathfinder, TilePosition};
#[macroquad::main("engineers")]
async fn main() {
    static ASSETS_DIR: Dir = include_dir!("assets");
    static MUSIC_DIR: Dir = include_dir!("assets/music");
    
    /*Load Assets*/
    let tileset_png = ASSETS_DIR.get_file("tileset.png").unwrap();
    let spritesheet_rock_png = ASSETS_DIR.get_file("spritesheet_rock.png").unwrap();
    let background_music_ogg = MUSIC_DIR.get_file("background_music.ogg").unwrap();
    let tileset = Texture2D::from_file_with_format(tileset_png.contents(), None);
    let engineer_anim = Texture2D::from_file_with_format(spritesheet_rock_png.contents(), None);
    let background_music = load_sound_from_bytes(background_music_ogg.contents()).await.unwrap();
    /****************************/
    
    /*Load JSON Tilemap*/
    let lib_rs = ASSETS_DIR.get_file("tiledmap.json").unwrap();
    let body = lib_rs.contents_utf8().unwrap();
    let map_cast: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
    /****************************/

    /*Generate Tiled Sprite List*/
    let mut sprite_list_vector: Vec<Box<dyn Sprite>> = get_tilemap_spritelist(tileset, &map_cast);

    let engy_sprite = Engineer {
        texture: engineer_anim,
        animated_sprite: AnimatedSprite::new(
            64,
            64,
            &[Animation {
                name: "idle".to_string(),
                row: 1,
                frames: 5,
                fps: 11,
            }],
            true,
        ),
        x: grid_to_world_coords(vec2(12.0, 4.0)).x,
        y: grid_to_world_coords(vec2(12.0, 4.0)).y,
        currentPath: Vec::new()
    };
    //Add Engineer Sprite
    sprite_list_vector.push(Box::new(engy_sprite));
    
    let mut camera_movement_var = 10.; // camera movement speed multiplier.

    let pathfinder_local = get_pathfinder(map_cast);

    play_sound(
        background_music,
        PlaySoundParams {
            looped: true,
            volume: 0.3,
        },
    );
    let mut lastTimeRecorded: f64 = get_time();
    loop {
        
        clear_background(BLACK);
        if camera_movement_var < 700. {
            camera_movement_var += 1.0
        } else {
            camera_movement_var = 0.0; //increment camera position.
        }
        let camera = Camera2D {
            zoom: vec2(
                1. / macroquad::window::screen_width() * 1.5,
                -1. / macroquad::window::screen_height() * 1.5,
            ),
            target: vec2(0.0 + camera_movement_var, 0.0 + camera_movement_var),
            ..Default::default()
        };
        set_camera(&camera);

        //Z-Index Sorting for render ordering
        sprite_list_vector.sort_by(|a, b| a.get_zindex().cmp(&b.get_zindex())); 
        let current_time = get_time();
        for sprite in sprite_list_vector.iter_mut() {
            sprite.update(current_time-lastTimeRecorded);
            sprite.draw(); //Draw all sprites in Sprite List
        }
        lastTimeRecorded=current_time;

        let (mouse_x, mouse_y) = mouse_position();
        if is_mouse_button_released(MouseButton::Left)
        {
            println!("mouse x: {}",mouse_x);
            println!("mouse y: {}",mouse_y);
            println!("camera_location X: {}",camera_movement_var);
            println!("camera_location Y: {}",camera_movement_var);
            let world_vec = camera.screen_to_world(vec2(mouse_x,mouse_y));
            println!("World X: {}",world_vec.x);
            println!("World Y: {}",world_vec.y);
            let grid_coords =world_to_grid_coords(world_vec);
            println!("Grid X: {}",world_to_grid_coords(world_vec).x-1.);
            println!("Grid Y: {}",world_to_grid_coords(world_vec).y);

            let tilePos = TilePosition{x:12, y:4};
            
            println!("tilePos X: {}",tilePos.x);
            println!("tilePos Y: {}",tilePos.y);
            let path =pathfinder_local.find_path(TilePosition{x: tilePos.x as i32, y: tilePos.y as i32}, TilePosition{x: grid_coords.x as i32, y: grid_coords.y as i32});
                // back to the original, concrete type.

            for sprite in sprite_list_vector.iter_mut() {
                sprite.updatePath(path.clone());//Draw all sprites in Sprite List
            }
            for pos in path {
                println!("Pos.x:{}, Pos.y:{}", pos.x, pos.y);
            }
            
        }


        next_frame().await;
    }
}

pub fn get_pathfinder(tilemap: tiledmap::TiledMap) -> Pathfinder
{
    Pathfinder::new(tilemap)
}

pub fn get_tilemap_spritelist(
    tileset: Texture2D,
    tilemap: &tiledmap::TiledMap,
) -> Vec<Box<dyn Sprite>>
{
    let mut sprite_list: Vec<Box<dyn Sprite>> = Vec::new();
    for layer_num in 0 as usize..tilemap.layers.len() as usize {
        for y in 0 as u32..tilemap.height as u32 {
            for x in 0 as u32..tilemap.width as u32 {
                let tile_sprite: TileSprite = TileSprite {
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
                };
                sprite_list.push(Box::new(tile_sprite));
            }
        }
    }
    sprite_list
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

        assert_eq!(pathfinder.tile_is_walkable(1,2), true);
    }

    #[test]
    fn test_collision_tile_2() {
        static ASSETS_DIR: Dir = include_dir!("assets");
        let lib_rs = ASSETS_DIR.get_file("tiledmap.json").unwrap();
        let body = lib_rs.contents_utf8().unwrap();
        let map_cast: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
        
        let pathfinder: Pathfinder = Pathfinder::new(map_cast);

        assert_eq!(pathfinder.tile_is_walkable(20,1), false);
    }
}