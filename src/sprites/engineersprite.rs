use super::super::pathfinding::pathfinder::TilePosition;
use super::sprite::{grid_to_world_coords, world_to_grid_coords, Sprite};
use macroquad::{color, experimental::animation::AnimatedSprite, prelude::*};

pub struct Engineer {
    pub texture: Texture2D,
    pub animated_sprite: AnimatedSprite,
    pub x: f32,
    pub y: f32,
    pub current_path: Vec<TilePosition>,
    pub previous_position: TilePosition,
    pub uuid: u32,
}

impl Engineer {
    pub fn is_within_bounds(&self, coords: Vec2) -> bool {
        if coords.x < self.x + 58.
            && coords.x > self.x
            && coords.y < self.y + 58.
            && coords.y > self.y
        {
            return true;
        }
        return false;
    }
    pub fn update(&mut self, time: f64) {
        if self.current_path.len() > 0 {
            let target = self.current_path[0];
            let speed = 75.; //units per second
            let mut speed_x = speed; //units per second
            let mut speed_y = speed/2.; //units per second
            let world_target = grid_to_world_coords(vec2(target.x as f32, target.y as f32));
            let x_dist = world_target.x - self.x;
            let y_dist = world_target.y - self.y;
            let mut x_unit = 1.;
            let mut y_unit = 1.;
            if x_dist < 0. {
                x_unit = -1.;
            }
            if y_dist < 0. {
                y_unit = -1.;
            }

            //Speed calculation
            let y_comp = self.previous_position.y - target.y;
            let x_comp = self.previous_position.x - target.x;
            if x_comp.abs() + y_comp.abs() == 1 {
                speed_x=(71.5/64.0)*0.666*speed; //approximate math
                speed_y=(71.5/64.0)*0.333*speed;
            }
            let distance_to_travel_x = (time as f32) * speed_x;
            let distance_to_travel_y = (time as f32) * speed_y;

            if (distance_to_travel_x > x_dist.abs()) {
                self.x = world_target.x;
            } else {
                self.x += x_unit * distance_to_travel_x;
            }
            if (distance_to_travel_y > y_dist.abs()) {
                self.y = world_target.y;
            } else {
                self.y += y_unit * distance_to_travel_y;
            }
            if (self.x < world_target.x + 0.0001)
                && (self.x > world_target.x - 0.0001)
                && (self.y < world_target.y + 0.0001)
                && (self.y > world_target.y - 0.0001)
            {
                if self.current_path.len() > 0 {
                    self.previous_position = self.current_path.remove(0);
                }
            }
        }
    }
    pub fn update_path(&mut self, path: Vec<TilePosition>) {
        self.current_path = path;
    }
}

impl Sprite for Engineer {
    fn get_zindex(&self) -> u32 {
        let grid_coords = self.get_tile_pos();
        2 + ((grid_coords.x + grid_coords.y) * 2.) as u32
    }
    fn get_tile_pos(&self) -> Vec2 {
        world_to_grid_coords(vec2(self.x + 8. as f32, self.y + 24. as f32))
    }
    fn draw(&mut self) {
        self.animated_sprite.update();

        draw_texture_ex(
            self.texture,
            self.x,
            self.y - 5.,
            color::WHITE,
            DrawTextureParams {
                source: Some(self.animated_sprite.frame().source_rect),
                dest_size: Some(self.animated_sprite.frame().dest_size),
                ..Default::default()
            },
        )
    }
}
