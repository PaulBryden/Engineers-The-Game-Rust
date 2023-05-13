use super::super::sprites::sprite::{ Sprite, SpriteID};
use crate::Vec2;
type SpriteMap = std::collections::HashMap<u32, SpriteID>;

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
