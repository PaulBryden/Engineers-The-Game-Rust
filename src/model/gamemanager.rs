use super::super::sprites::sprite::{grid_to_world_coords, world_to_grid_coords, Sprite, SpriteID};
use super::requests::Request;
use super::requests::RequestQueue;
use super::gamestate::GameState;
use crate::model::requests::RequestImpl;
use crate::Pathfinder;
use crate::SpriteMoveRequest;
use crate::TilePosition;
use crate::Vec2;
use crate::sprites::engineersprite::Engineer;
use include_dir::include_dir;
use macroquad::texture::Texture2D;

type Tick = u32;

pub enum RequestStatus {
    Synchronized,
    Desynchronized,
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
        
    pub fn addLocalRequest(&mut self, request: Request) -> RequestStatus {
        //Send Request Over Network Here
        /*
        */
        return self.addRequest(request);
       
    }
    pub fn addNetworkRequest(&mut self, request: Request) -> RequestStatus {
        return self.addRequest(request);
       
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
                    self.addLocalRequest(request);
                }
            }
        }
    }
}
