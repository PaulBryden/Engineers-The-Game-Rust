
use macroquad::{
    prelude::*,
};

pub trait Sprite {
    fn get_zindex(&self) -> u32;
    fn get_tile_pos(&self) -> Vec2;
    fn draw(&mut self);
}

pub fn grid_to_world_coords(grid_pos: Vec2) -> Vec2 {
    vec2(
        ((grid_pos.x as f32 * 64. / 2.0) - (grid_pos.y as f32 * 64. / 2.0)) + 8.,
        ((grid_pos.y as f32 * 32. / 2.0) + (grid_pos.x as f32 * 32. / 2.0)) - 8.,
    )
}
pub fn world_to_grid_coords(world_pos: Vec2) -> Vec2 //In Testing
{
    vec2(
        world_pos.x + (2. * world_pos.y) / 64.,
        (world_pos.x + (2. * world_pos.y) / 64.) + (world_pos.x / 32.),
    )
}