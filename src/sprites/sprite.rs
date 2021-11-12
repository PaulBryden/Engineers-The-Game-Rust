use macroquad::prelude::*;

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
    let worldX=world_pos.x-8.0;
    let worldY=world_pos.y+8.0;
    vec2(
        (worldX + (2. * worldY)) / 64.,
        (worldY/16.)-((worldX+ (2. * worldY)) / 64.),
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