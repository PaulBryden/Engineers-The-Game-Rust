use crate::model::gamestate::GameState;
use crate::model::gamemanager::GameManager;
use include_dir::include_dir;
use include_dir::Dir;
use macroquad::prelude::*;
use quad_net::quad_socket::client::QuadSocket;

mod pathfinding {
    pub mod pathfinder;
}
use macroquad::{
    audio::{load_sound_from_bytes, play_sound, PlaySoundParams}
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
use quad_net::web_socket::WebSocket;
use sprites::sprite::{world_to_grid_coords, SpriteID};
use model::requests::*;
#[macroquad::main("engineers")]
async fn main() {
    static ASSETS_DIR: Dir = include_dir!("assets");
    static MUSIC_DIR: Dir = include_dir!("assets/music");
    /*Load Assets*/
    let tileset_png = ASSETS_DIR.get_file("tileset.png").unwrap();
    let background_music_ogg = MUSIC_DIR.get_file("background_music.ogg").unwrap();
    let tileset = Texture2D::from_file_with_format(tileset_png.contents(), None);
    let background_music = load_sound_from_bytes(background_music_ogg.contents())
        .await
        .unwrap();

    /*Load JSON Tilemap*/
    let lib_rs = ASSETS_DIR.get_file("tiledmap.json").unwrap();
    let body = lib_rs.contents_utf8().unwrap();
    let tilemap_struct: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
    /****************************/

    /*Seed random number generator*/
    rand::srand((get_time() * 99999.99) as u64);
    /**************************/

    /*Generate Tiled Sprite List*/
    let sprite_map_store: std::collections::HashMap<u32,SpriteID> = tiledmap::get_tilemap_spritelist(tileset, &tilemap_struct);
    let mut render_list: Vec<u32> = Vec::new();
    for (uuid, _sprite) in sprite_map_store.iter()
    {
        render_list.push(*uuid);
    }

    /*Create Game State*/
    let game_state: GameState = GameState{sprite_map:sprite_map_store, sprite_uuid_list:render_list, selected_entity: 0};

    /*initialize web socket connection to server */
    #[cfg(not(target_arch = "wasm32"))]
    let mut socket = WebSocket::connect("ws://127.0.0.1:3012/12345").unwrap();
    #[cfg(target_arch = "wasm32")]
    let mut socket = WebSocket::connect("ws://127.0.0.1:3012/12345").unwrap();

    {
        while socket.connected() == false {
            next_frame().await;
        }
    }
    let mut connected=false;
    while(!connected)
    {
        println!("Waiting on Data...");

        while let Some(event) =  socket.try_recv(){
            println!("Received {:?}", event);
            connected=true;
        }
        socket.send_text(("1"));
        next_frame().await;
    }

    /*Create Game Manager*/
    let mut game_manager: GameManager = GameManager{socket: socket, requests: RequestQueue::default(), game_state_history: std::collections::HashMap::new(), current_game_state: game_state, last_tick: 0, pathfinder:  Pathfinder::new(tilemap_struct)};
 
    /*Initialize Game State By executing first tick - 0 */
    let mut last_tick_time: f64 = get_time();
    let mut tick_count =0;
    game_manager.current_game_state.process_tick(tick_count);
       
    /*Generate Random Create Requests for Engineers */
    for _i in 0..8 {
        let position = vec2(rand::gen_range::<f32>(1.0, 10.0) as f32, rand::gen_range::<f32>(1.0, 10.0) as f32);
        let uuid: u32 = rand::rand();
        let request: Request = Request::SpriteCreate(SpriteCreateRequest {tick:40 - position.x as u32, sprite_uuid: uuid, position: TilePosition{x: position.x as i32,y: position.y as i32}, sprite_type: SpriteType::Engineer});
        game_manager.addLocalRequest(request);
    }

    /*Play Music */
    play_sound(
        background_music,
        PlaySoundParams {
            looped: true,
            volume: 0.1,
        },
    );

    /* Render/Tick Loop */
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

        /* Mouse/Touch Handling */
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

        
        /*Get current game clock time*/
        let current_time = get_time();
        game_manager.getNetworkRequests();
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
