use include_dir::include_dir;
use include_dir::Dir;
use macroquad::prelude::*;
use macroquad::{
    experimental::{
        animation::{AnimatedSprite, Animation},
    },
};
pub mod sprite;
pub mod tiledmap;
use sprite::{Engineer, Sprite, TileSprite};
#[macroquad::main("engineers")]
async fn main() {
    let tileset = load_texture("assets/tileset.png").await.unwrap();
    let engineer_anim = load_texture("assets/spritesheet_rock.png").await.unwrap();

    /*Load JSON Tilemap*/
    static PROJECT_DIR: Dir = include_dir!("assets");
    let lib_rs = PROJECT_DIR.get_file("tiledmap.json").unwrap();
    let body = lib_rs.contents_utf8().unwrap();
    let map_cast: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
    /****************************/

    /*Generate Tiled Sprite List*/
    let mut sprite_list_vector: Vec<Box<dyn Sprite>> = get_tilemap_spritelist(tileset, map_cast);
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
        x: sprite::grid_to_world_coords(vec2(1.0, 1.0)).x,
        y: sprite::grid_to_world_coords(vec2(1.0, 1.0)).y,
    }));
    let mut camera_movement_var = 10.; // camera movement speed multiplier.
    loop {
        clear_background(BLACK);
        if camera_movement_var < 700. {
            camera_movement_var += 1.0
        } else {
            camera_movement_var = 0.0; //increment camera position.
        }
        set_camera(&Camera2D {
            zoom: vec2(
                1. / macroquad::window::screen_width() * 1.5,
                -1. / macroquad::window::screen_height() * 1.5,
            ),
            target: vec2(0.0 + camera_movement_var, 0.0 + camera_movement_var),
            ..Default::default()
        });

        sprite_list_vector.sort_by(|a, b| a.get_zindex().cmp(&b.get_zindex())); //Z-Index sort for rendering

        for sprite in sprite_list_vector.iter_mut() {
            sprite.draw();
        }
        next_frame().await;
    }
}

pub fn get_tilemap_spritelist(
    tileset: Texture2D,
    tilemap: tiledmap::TiledMap,
) -> Vec<Box<dyn Sprite>>
//vector of sprites for Z index rendering
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
