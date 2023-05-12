use super::super::pathfinding::pathfinder::TilePosition;
use super::sprite::{grid_to_world_coords, world_to_grid_coords, Sprite};
use macroquad::{color, experimental::animation::AnimatedSprite, prelude::*};
use macroquad::{
    experimental::animation::{Animation},
};

#[derive(Clone)]
pub struct Engineer {
    pub texture: Texture2D,
    pub animated_sprite: AnimatedSprite,
    pub selected_texture: Texture2D,
    pub x: i32,
    pub y: i32,
    pub current_path: Vec<TilePosition>,
    pub previous_position: TilePosition,
    pub uuid: u32,
    pub selected: bool,
    movement_tick_counter: i32,
    ticks_to_move_one_square: i32,
}

impl Engineer {

    pub fn new(x: i32, y: i32, width: i32, height: i32, spritesheet: Texture2D, uuid: u32, selected_texture: Texture2D ) -> Self 
    {
        Self {texture: spritesheet, animated_sprite: AnimatedSprite::new(
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
        selected_texture:selected_texture,
    x: x, y: y, current_path: Vec::new(), previous_position: TilePosition{x:x,y:y},  uuid:uuid, selected:false,movement_tick_counter: 0, ticks_to_move_one_square: 10}
    }

    pub fn get_animation_direction(current_position: &TilePosition, next_position: &TilePosition) -> usize
    {
        let x_differential= current_position.x-next_position.x;
        let y_differential = current_position.y-next_position.y;
        if x_differential==1 && y_differential==1
        {
            return 0;
        }else if x_differential==1 && y_differential==0
        {
            return 1;
        }else if x_differential==1 && y_differential==-1
        {
            return 2;
        }else if x_differential==0 && y_differential==-1
        {
            return 3;
        }else if x_differential==-1 && y_differential==-1
        {
            return 4;
        }else if x_differential==-1 && y_differential==0
        {
            return 5;
        }else if x_differential==-1 && y_differential==1
        {
            return 6;
        }else if x_differential==0 && y_differential==1
        {
            return 7;
        }
        else
        {
            return 3;
        }
    }
    pub fn is_within_bounds(&self, coords: Vec2) -> bool {
        let sprite_world_coords = grid_to_world_coords( Vec2::new(self.x as f32, self.y as f32));

        if coords.x < sprite_world_coords.x + 58.
            && coords.x > sprite_world_coords.x
            && coords.y < sprite_world_coords.y + 58.
            && coords.y > sprite_world_coords.y
        {
            return true;
        }
        return false;
    }
    
    pub fn tick(&mut self, time: u32) {
        self.handle_tick();
    }
    pub fn update_path(&mut self, path: Vec<TilePosition>) {
        if self.current_path.len() > 0 {
            self.previous_position = TilePosition{x:self.get_tile_pos().x as i32, y:self.get_tile_pos().y as i32};
        }
        self.movement_tick_counter=0;
        self.current_path = path;
    }

    fn handle_tick(&mut self) {
        self.animated_sprite.set_animation(Engineer::get_animation_direction(&TilePosition { x: self.x, y: self.y }, &self.current_path.get(0).or_else(||Some(&self.previous_position
        )).unwrap()));

        self.movement_tick_counter+=1;
        if(self.movement_tick_counter==self.ticks_to_move_one_square)
        {
            self.movement_tick_counter=0;
            if(self.current_path.len()>0)
            {
                self.previous_position= TilePosition{x:self.x,y: self.y};
                self.x=self.current_path[0].x;
                self.y=self.current_path[0].y;
                self.current_path.remove(0);
            }
        }
    }
}



impl Sprite for Engineer {
    fn get_zindex(&self) -> u32 {
        let grid_coords = self.get_tile_pos();
        1 + ((grid_coords.x + grid_coords.y) * 2.) as u32
    }
    fn get_tile_pos(&self) -> Vec2 {
       vec2(self.x as f32, self.y as f32)
    }
    fn draw(&mut self) {
        self.animated_sprite.update();
        let render_location: Vec2;
        if (self.current_path.get(0).is_some()) {
            let x_offset: f32 = (self.current_path[0].x as f32 - self.x as f32) * (self.movement_tick_counter as f32 / self.ticks_to_move_one_square as f32);
            let y_offset: f32 = (self.current_path[0].y as f32 - self.y as f32) * (self.movement_tick_counter as f32/ self.ticks_to_move_one_square as f32);
            render_location = grid_to_world_coords(Vec2{x:self.x as f32+ x_offset, y: self.y as f32 + y_offset});
        } else {
            render_location = grid_to_world_coords(Vec2{x:self.x as f32, y:self.y as f32});
        }
        draw_texture_ex(
            self.texture,
            render_location.x as f32,
            render_location.y as f32- 5.,
            color::WHITE,
            DrawTextureParams {
                source: Some(self.animated_sprite.frame().source_rect),
                dest_size: Some(self.animated_sprite.frame().dest_size),
                ..Default::default()
            },
        );
        if(self.selected)
        {
            draw_texture_ex(
                self.selected_texture,
                render_location.x as f32,
                render_location.y as f32,
                color::WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(64.0, 64.0)),
                    source: Some(Rect::new(0., 0., 64., 64.)),
                    ..Default::default()
                },
            );
        }
    }
}
