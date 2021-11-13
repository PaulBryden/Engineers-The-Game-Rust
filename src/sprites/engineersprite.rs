use super::super::pathfinding::pathfinder::TilePosition;
use super::sprite::{grid_to_world_coords, world_to_grid_coords, Sprite};
use macroquad::{color, experimental::animation::AnimatedSprite, prelude::*};
use std::any::Any;

pub struct Engineer {
    pub texture: Texture2D,
    pub animated_sprite: AnimatedSprite,
    pub x: f32,
    pub y: f32,
    pub currentPath: Vec<TilePosition>,
}

impl Engineer {
    pub fn update(&mut self, time: f64) {}
}

impl Sprite for Engineer {
    fn update(&mut self, time: f64) {
        if self.currentPath.len() > 0 {
            let mut target = self.currentPath[0];
            let mut speed = 25.; //units per second
            let mut world_target = grid_to_world_coords(vec2(target.x as f32, target.y as f32));
            let mut x_dist = world_target.x - self.x;
            let mut y_dist = world_target.y - self.y;
            let mut x_unit = 1.;
            let mut y_unit = 1.;
            if (x_dist < 0.) {
                x_unit = -1.;
            }
            if (y_dist < 0.) {
                y_unit = -1.;
            }
            let mut distance_to_travel_x = (time as f32) * speed;
            let mut distance_to_travel_y = (time as f32) * speed;
            if(x_unit*x_dist<distance_to_travel_x)
            {
                distance_to_travel_x-= (x_unit*x_dist);
                distance_to_travel_y-= (y_unit*x_dist);
                let old_world_target = world_target;
                self.currentPath.remove(0);
                if(self.currentPath.len()>0)
                {
                target= self.currentPath[0];
                world_target = grid_to_world_coords(vec2(target.x as f32, target.y as f32));
                 x_dist = world_target.x - old_world_target.x;
                 y_dist = world_target.y - old_world_target.y;
                 if (x_dist < 0.) {
                    x_unit = -1.;
                }
                if (y_dist < 0.) {
                    y_unit = -1.;
                }
                self.x = old_world_target.x + x_unit* distance_to_travel_x;
                self.y = old_world_target.y + y_unit * distance_to_travel_y; 
                }
                else
                {
                    self.x = old_world_target.x ;
                    self.y = old_world_target.y ;
                }
            }
            else
            {
                self.x +=x_unit * distance_to_travel_x;
                self.y +=y_unit * distance_to_travel_y; 
            }
            if (self.x < world_target.x + 0.2)
                && (self.x > world_target.x - 0.2)
                && (self.y < world_target.y + 0.2)
                && (self.y > world_target.y - 0.2)
            {
                if(self.currentPath.len()>0)
                {
                    self.currentPath.remove(0);
                }
            }
        }
    }
    fn updatePath(&mut self, path: Vec<TilePosition>) {
        self.currentPath = path;
    }
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
    fn get_zindex(&self) -> u32 {
        let grid_coords = world_to_grid_coords(vec2(self.x, self.y));
        3 + ((grid_coords.x + grid_coords.y) / 2.0) as u32
    }
    fn get_tile_pos(&self) -> Vec2 {
        world_to_grid_coords(vec2(self.x as f32, self.y as f32))
    }
    fn draw(&mut self) {
        self.animated_sprite.update();

        draw_texture_ex(
            self.texture,
            self.x,
            self.y - 10.,
            color::WHITE,
            DrawTextureParams {
                source: Some(self.animated_sprite.frame().source_rect),
                dest_size: Some(self.animated_sprite.frame().dest_size),
                ..Default::default()
            },
        )
    }
}
