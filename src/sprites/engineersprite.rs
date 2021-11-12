
use macroquad::{
    color,
    prelude::*,
    experimental::{
        animation::{AnimatedSprite}
    },
};
use super::sprite::{Sprite,world_to_grid_coords};
pub struct Engineer {
    pub texture: Texture2D,
    pub animated_sprite: AnimatedSprite,
    pub x: f32,
    pub y: f32,
}


impl Sprite for Engineer {
    fn get_zindex(&self) -> u32 {
        let grid_coords = world_to_grid_coords(vec2(self.x, self.y));

        (grid_coords.x + grid_coords.y+1.) as u32
    }
    fn get_tile_pos(&self) -> Vec2 {
        vec2(self.x as f32, self.y as f32)
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
