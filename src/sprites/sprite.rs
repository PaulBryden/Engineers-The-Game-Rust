use macroquad::prelude::*;
use super::engineersprite::Engineer;
use super::tilesprite::TileSprite;
pub trait Sprite {
    fn get_zindex(&self) -> u32;
    fn get_tile_pos(&self) -> Vec2;
    fn draw(&mut self);
}
pub enum SpriteID {
    Engineer(Engineer),
    Tile(TileSprite)
}

impl Sprite for SpriteID
{

fn get_zindex(&self) -> u32 { 
    match self{
        SpriteID::Engineer(engineer_entity) => engineer_entity.get_zindex(),
        SpriteID::Tile(tile_entity) => tile_entity.get_zindex()
    }
}
fn get_tile_pos(&self) -> macroquad::math::Vec2 {
    match self{
        SpriteID::Engineer(engineer_entity) => engineer_entity.get_tile_pos(),
        SpriteID::Tile(tile_entity) => tile_entity.get_tile_pos()
    }
 }
fn draw(&mut self) {
    match self{
        SpriteID::Engineer(engineer_entity) => engineer_entity.draw(),
        SpriteID::Tile(tile_entity) => tile_entity.draw()
    }
 }
}

pub fn grid_to_world_coords(grid_pos: Vec2) -> Vec2 {
    vec2(
        ((grid_pos.x as f32 * 64. / 2.0) - (grid_pos.y as f32 * 64. / 2.0)) + 8.,
        ((grid_pos.y as f32 * 32. / 2.0) + (grid_pos.x as f32 * 32. / 2.0)) - 8.,
    )
}

pub fn world_to_grid_coords(world_pos: Vec2) -> Vec2 //In Testing
{
    let world_x=world_pos.x-8.0;
    let world_y=world_pos.y+8.0;
    vec2(
        (world_x + (2. * world_y)) / 64.,
        (world_y/16.)-((world_x+ (2. * world_y)) / 64.),
    )
}

#[cfg(test)]
mod tests {
    use macroquad::prelude::*;
    use super::*;
    #[test]
    fn grid_to_world_and_back_test() {
        let pos=vec2(3.0,5.0);
        assert_eq!(world_to_grid_coords(grid_to_world_coords(pos)), pos);
    }

    #[test]
    fn world_to_grid_and_back_test() {
        let pos=vec2(30.0,50.0);
        assert_eq!(grid_to_world_coords(world_to_grid_coords(pos)), pos);
    }
}