use include_dir::include_dir;
use include_dir::Dir;
use macroquad::prelude::*;
use macroquad::{
    audio::{load_sound, play_sound, stop_sound, PlaySoundParams, Sound},
    experimental::{
        animation::{AnimatedSprite, Animation},
    },
    input::*
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
#[macroquad::main("engineers")]
async fn main() {
    let tileset = load_texture("assets/tileset.png").await.unwrap();
    let engineer_anim = load_texture("assets/spritesheet_rock.png").await.unwrap();
    let background_music = load_sound("assets/music/background_music.ogg")
        .await
        .unwrap();
    /*Load JSON Tilemap*/
    static PROJECT_DIR: Dir = include_dir!("assets");
    let lib_rs = PROJECT_DIR.get_file("tiledmap.json").unwrap();
    let body = lib_rs.contents_utf8().unwrap();
    let map_cast: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
    /****************************/

    /*Generate Tiled Sprite List*/
    let mut sprite_list_vector: Vec<Box<dyn Sprite>> = get_tilemap_spritelist(tileset, map_cast);

    //Add Engineer Sprite
    sprite_list_vector.push(Box::new(Engineer {
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
    }));
    
    let mut camera_movement_var = 10.; // camera movement speed multiplier.



    play_sound(
        background_music,
        PlaySoundParams {
            looped: true,
            volume: 0.3,
        },
    );

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

        for sprite in sprite_list_vector.iter_mut() {
            sprite.draw(); //Draw all sprites in Sprite List
        }

        let (mouse_x, mouse_y) = mouse_position();
        if(is_mouse_button_released(MouseButton::Left))
        {
            println!("mouse x: {}",mouse_x);
            println!("mouse y: {}",mouse_y);
            println!("camera_location X: {}",camera_movement_var);
            println!("camera_location Y: {}",camera_movement_var);
            let xWorld = (mouse_x- macroquad::window::screen_width()/2.) +camera_movement_var;
            let yWorld = (mouse_y- macroquad::window::screen_height()/2.) +camera_movement_var;
            let mut worldVec = camera.screen_to_world(vec2(mouse_x,mouse_y));
            println!("World X: {}",worldVec.x);
            println!("World Y: {}",worldVec.y);
            println!("Grid X: {}",world_to_grid_coords(worldVec).x-1.);
            println!("Grid Y: {}",world_to_grid_coords(worldVec).y);

        }

        next_frame().await;
    }
}

pub fn get_tilemap_spritelist(
    tileset: Texture2D,
    tilemap: tiledmap::TiledMap,
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
