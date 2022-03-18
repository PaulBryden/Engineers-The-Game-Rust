
use macroquad::{
    prelude::*,
};
use super::sprite::{Sprite};
#[derive(Clone)]
pub struct TileSprite {
    pub texture: Texture2D,
    pub frame_number: u32,
    pub x: u32,
    pub y: u32,
    pub width: f32,
    pub height: f32,
    pub layer: u32,
    pub uuid: u32
}


impl Sprite for TileSprite {
    
    fn get_zindex(&self) -> u32 {
        if self.layer == 1 ||  self.frame_number>15{
            0
        } else {
            2 + ((self.x + self.y)*2) as u32
        }
    }
    fn get_tile_pos(&self) -> Vec2 {
        vec2(self.x as f32, self.y as f32)
    }
    fn draw(&mut self) {
        let pos = vec2(
            (self.x as f32 * self.width / 2.0) - (self.y as f32 * self.width / 2.0),
            (self.y as f32 * self.height / 2.0) + (self.x as f32 * self.height / 2.0),
        );
        let spr_rect = self.sprite_rect();
        draw_texture_ex(
            self.texture,
            pos.x,
            pos.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(64.0, 64.0)),
                source: Some(Rect::new(
                    spr_rect.x - 1.0,
                    spr_rect.y - 1.0 + 16.0,
                    spr_rect.w,
                    spr_rect.h,
                )),
                ..Default::default()
            },
        );
    }
}

impl TileSprite {
    fn sprite_rect(&self) -> Rect {
        let ix = self.frame_number;
        let sw = 64 as f32;
        let sh = 64 as f32;
        let sx = (ix % 20) as f32 * (sw + 0 as f32) + 0 as f32;
        let sy = (ix / 20) as f32 * (sh + 0 as f32) + 0 as f32;
        // TODO: configure tiles margin
        Rect::new(sx + 1.1, sy + 1.1, sw - 2.2, sh - 2.2)
    }
}
