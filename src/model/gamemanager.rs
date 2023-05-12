use super::super::sprites::sprite::{grid_to_world_coords, world_to_grid_coords, Sprite, SpriteID};
use super::requests::Request;
use super::requests::RequestQueue;
use crate::model::requests::RequestImpl;
use crate::Pathfinder;
use crate::SpriteMoveRequest;
use crate::TilePosition;
use crate::Vec2;
use crate::sprites::engineersprite::Engineer;
use macroquad::prelude::draw_texture_ex;
use include_dir::include_dir;
use include_dir::Dir;
use macroquad::texture::Texture2D;

type SpriteMap = std::collections::HashMap<u32, SpriteID>;
type Tick = u32;

pub enum RequestStatus {
    Synchronized,
    Desynchronized,
}
#[derive(Clone, Default)]
pub struct GameState {
    pub sprite_map: SpriteMap,
    pub sprite_uuid_list: Vec<u32>,
    pub selected_entity: u32,
}
impl GameState {
    pub fn sort_by_z_index(&mut self) {
        let mut sprite_uuid_copy = self.sprite_uuid_list.clone();
        sprite_uuid_copy.sort_by(|a, b| {
            let ordering = self
                .sprite_map
                .get_mut(a)
                .unwrap()
                .get_zindex()
                .cmp(&self.sprite_map.get(b).unwrap().get_zindex());
            if ordering == std::cmp::Ordering::Equal {
                let mut a_val = 0;
                let mut b_val = 0;
                match self.sprite_map.get(a).unwrap() {
                    SpriteID::Tile(sprite_move_request) => a_val = 1,
                    Default => b_val = b_val,
                }
                match self.sprite_map.get(b).unwrap() {
                    SpriteID::Tile(sprite_move_request) => b_val = 1,
                    Default => b_val = b_val,
                }
                return a_val.cmp(&b_val);
            } else {
                return ordering;
            }
        });
        self.sprite_uuid_list = sprite_uuid_copy;
    }

    pub fn render(&mut self) {
        self.sort_by_z_index();
        for uuid in &self.sprite_uuid_list {
            let sprite = self.sprite_map.get_mut(&uuid).unwrap();
            sprite.draw(); //Draw all sprites in Sprite List
        }
    }
    pub fn is_sprite_within_bounds(&mut self, mouse_coords: Vec2) -> Option<u32> {
        let mut selected = 0;
        for uuid in &self.sprite_uuid_list {
            let sprite = self.sprite_map.get_mut(&uuid).unwrap();
            match sprite {
                SpriteID::Engineer(engineer_entity) => {
                    if engineer_entity.is_within_bounds(mouse_coords) {
                        selected = engineer_entity.uuid;
                    }
                }
                _default => {}
            };
        }
        if selected != 0 {
            return Some(selected);
        }
        return None;
    }

    pub fn process_tick(&mut self, tick: u32) {
        for uuid in &self.sprite_uuid_list {
            let sprite = self.sprite_map.get_mut(&uuid).unwrap();
            match sprite {
                SpriteID::Engineer(engineer_entity) => {
                    engineer_entity.tick(1); //each tick is 20ms
                }
                SpriteID::Tile(_tile_entity) => {}
            }
        }
    }

    pub fn mark_new_selected_sprite(&mut self, uuid: u32) {
        if self.selected_entity != 0 {
            let sprite = self.sprite_map.get_mut(&self.selected_entity).unwrap();
            match sprite {
                SpriteID::Engineer(engineer_entity) => {
                    engineer_entity.selected = false;
                }
                _default => {}
            }
        }
        let sprite = self.sprite_map.get_mut(&uuid).unwrap();
        match sprite {
            SpriteID::Engineer(engineer_entity) => {
                engineer_entity.selected = true;
                self.selected_entity = uuid;
            }
            _default => {}
        }
    }
}

pub struct GameManager {
    pub requests: RequestQueue, //Requests are global and maintain as much history as game_state_history.
    pub game_state_history: std::collections::HashMap<Tick, GameState>,
    pub current_game_state: GameState,
    pub last_tick: u32,
    pub pathfinder: Pathfinder,
}

impl GameManager {
    pub fn process_tick(&mut self, tick: u32) {
        let mut local_tick = tick;
        if (local_tick > self.last_tick) {
            self.last_tick = local_tick;
            self.process_tick_work(local_tick);
        } else {
            self.current_game_state = (*self.game_state_history.get(&tick).unwrap()).clone();
            while (local_tick <= self.last_tick) {
                self.process_tick_work(tick);
                local_tick = local_tick + 1;
            }
        }
    }
    pub fn process_tick_work(&mut self, tick: u32) {
        let requests_to_be_processed = self.requests.GetRequestsOfParticularTick(tick);
        /*Process Requests Here*/
        for request in requests_to_be_processed {
            self.process_request(&request);
        }
        /***********************/
        self.current_game_state.process_tick(tick);
        self.game_state_history
            .insert(tick, self.current_game_state.clone());
        if (tick > 47)
        {
            self.game_state_history.remove(&(tick - 47));
        }
    }

    pub fn process_request(&mut self, request: &Request) {
        match request {
            Request::SpriteMove(sprite_move) => {
                match &mut self
                    .current_game_state
                    .sprite_map
                    .get_mut(&self.current_game_state.selected_entity)
                    .unwrap()
                {
                    SpriteID::Engineer(engineer_entity) => {
                        let mut path = self.pathfinder.find_path(
                            TilePosition {
                                x: engineer_entity.get_tile_pos().x as i32,
                                y: engineer_entity.get_tile_pos().y as i32,
                            },
                            TilePosition {
                                x: (sprite_move.position.x ) as i32,
                                y: (sprite_move.position.y) as i32,
                            },
                        );
                        engineer_entity.update_path(std::mem::take(&mut path))
                    }
                    _default => {}
                }
            }
            Request::SpriteCreate(request) => {
                match request.sprite_type {
                    crate::model::requests::SpriteType::Engineer =>
                    {
                        let engy_sprite =  Engineer::new(request.position.x, request.position.y, 64, 64, Texture2D::from_file_with_format(include_dir!("assets").get_file("spritesheet_rock.png").unwrap().contents(),None), request.sprite_uuid, Texture2D::from_file_with_format(include_dir!("assets").get_file("selected.png").unwrap().contents(), None));
                        self.current_game_state.sprite_map.insert(request.sprite_uuid, SpriteID::Engineer(engy_sprite));
                        self.current_game_state.sprite_uuid_list.push(request.sprite_uuid);
                        
                    }
                    _default => {}
            }
        }
    }
}

    pub fn render(&mut self) {
        self.current_game_state.render();
    }
    pub fn addRequest(&mut self, request: Request) -> RequestStatus {
        self.requests.AddRequest(request);
        if (request.get_tick() < self.last_tick && (self.last_tick - request.get_tick()) < 45)
        //Received old request. Time to synchronize
        {
            self.process_tick(request.get_tick());
            return RequestStatus::Synchronized;
        } else if self.last_tick > 45 && request.get_tick() < self.last_tick &&  (self.last_tick - request.get_tick()) > 45
        //Request is too old. Game State is desynchronized.
        {
            return RequestStatus::Desynchronized;
        } else {
            if (self.last_tick > 47) {
                self.requests
                    .PurgeRequestsOlderThanTick(self.last_tick - 47);
            }
            return RequestStatus::Synchronized;
        }
    }
    pub fn mouse_clicked(&mut self, mouse_coords: Vec2) {
        let selected_unit_uuid = self
            .current_game_state
            .is_sprite_within_bounds(mouse_coords); //if it is within bounds, selection has occured. If it is not within bounds, move or other operation has been requested.
        match selected_unit_uuid {
            Some(unit_uuid) => {
                self.current_game_state.mark_new_selected_sprite(unit_uuid);
            }
            None => {
                /*Move Request*/
                if (self.current_game_state.selected_entity != 0) {
                    let request = Request::SpriteMove(SpriteMoveRequest {
                        tick: self.last_tick + 10,
                        sprite_uuid: self.current_game_state.selected_entity,
                        position: TilePosition {
                            x: (world_to_grid_coords(mouse_coords).x - 1.0) as i32,
                            y: (world_to_grid_coords(mouse_coords).y - 0.5) as i32,
                        },
                    });
                    self.addRequest(request);
                }
            }
        }
    }
}
