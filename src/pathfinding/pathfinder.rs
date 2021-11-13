use super::super::tiledmap;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;


pub struct Pathfinder {
    tilemap: tiledmap::TiledMap,
}

#[derive(Debug, Copy, Clone)]
pub struct TilePosition {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
pub struct TilePositionKeyEntry {
    position: TilePosition,
    key: String,
}

pub fn tile_position_to_key(x: i32, y: i32) -> String {
    [x.to_string(), y.to_string()].join("x")
}

impl Pathfinder {
    pub fn new(map: tiledmap::TiledMap) -> Pathfinder {
        Pathfinder { tilemap: map }
    }
    pub fn tile_is_walkable(&self, x: i32, y: i32) -> bool {
        if x >= 0 && y >= 0 {
            //remember to decrement the tile number.
            !self.tilemap.tilesets[0].tiles
                [self.tilemap.layers[0].get_tile_at(x as u32, y as u32) as usize - 1 as usize]
                .properties[0]
                .value
        } else {
            return false;
        }
    }

    pub fn find_path(&self, start: TilePosition, target: TilePosition) -> Vec<TilePosition> {
        let mut path: Vec<TilePosition> = Vec::new();
        if !self.tile_is_walkable(target.x, target.y) {
            return path;
        }
        if !self.tile_is_walkable(start.x, start.y) {
            return path;
        }
        let mut queue: VecDeque<TilePosition> = VecDeque::new();
        let mut parent_for_key: HashMap<String, TilePositionKeyEntry> = HashMap::new();

        let start_key = tile_position_to_key(start.x, start.y);
        let target_key = tile_position_to_key(target.x, target.y);
        parent_for_key.insert(
            start_key.clone(),
            TilePositionKeyEntry {
                key: "".to_string(),
                position: TilePosition { x: 0, y: 0 },
            },
        );
        queue.push_back(start);
        while queue.len() > 0 {
            let pos: TilePosition = queue.pop_front().unwrap();
            let current_key = tile_position_to_key(pos.x, pos.y);

            if current_key == target_key {
                break;
            }

            let mut neighbours: Vec<TilePosition> = Vec::new();
            if pos.y - 1 >= 0 {
                neighbours.push(TilePosition {
                    x: pos.x,
                    y: pos.y - 1,
                });
            }
            if pos.x - 1 >= 0 {
                neighbours.push(TilePosition {
                    x: pos.x - 1,
                    y: pos.y,
                });
            }
            if pos.x + 1 < self.tilemap.layers[0].width as i32 {
                neighbours.push(TilePosition {
                    x: pos.x + 1,
                    y: pos.y,
                });
            }
            if pos.y + 1 < self.tilemap.layers[0].height as i32 {
                neighbours.push(TilePosition {
                    x: pos.x,
                    y: pos.y + 1,
                });
            }

            for neighbour in neighbours {
                if !self.tile_is_walkable(neighbour.x, neighbour.y) {
                    continue;
                }
                let key = tile_position_to_key(neighbour.x, neighbour.y);
                if parent_for_key.contains_key(&key) {
                    continue;
                }
                parent_for_key.insert(
                    key,
                    TilePositionKeyEntry {
                        key: current_key.clone(),
                        position: neighbour,
                    },
                );
                queue.push_back(neighbour);
            }
        }
        let mut current_key = target_key;
        let mut current_pos;

        while current_key != start_key {
            let tile_pos_key_entry: TilePositionKeyEntry = parent_for_key[&current_key].clone();
            current_key = tile_pos_key_entry.key;
            current_pos = tile_pos_key_entry.position;
            path.push(current_pos.clone());
        }
        path.reverse();

        return path;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use include_dir::include_dir;
    use include_dir::Dir;
    use std::time::Instant;
    use std::time::Duration;
    #[test]
    fn pathfinding_large_test() {
        static PROJECT_DIR: Dir = include_dir!("assets");
        let lib_rs = PROJECT_DIR.get_file("tiledmap.json").unwrap();
        let body = lib_rs.contents_utf8().unwrap();
        let map_cast: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
        let pathfinder: Pathfinder = Pathfinder::new(map_cast);

        let path = pathfinder.find_path(TilePosition { x: 1, y: 1 }, TilePosition { x: 45, y: 45 });
        println!("Pos.length: {}", path.len());
        for pos in path {
            println!("Pos.x:{}, Pos.y:{}", pos.x, pos.y);

            assert_eq!(pathfinder.tile_is_walkable(pos.x, pos.y), true);
        }
        assert_eq!(false, false);
    }
    #[test]
    fn pathfinding_same_start_end_tile_test() {
        static PROJECT_DIR: Dir = include_dir!("assets");
        let lib_rs = PROJECT_DIR.get_file("tiledmap.json").unwrap();
        let body = lib_rs.contents_utf8().unwrap();
        let map_cast: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
        let pathfinder: Pathfinder = Pathfinder::new(map_cast);

        let path = pathfinder.find_path(TilePosition { x: 1, y: 1 }, TilePosition { x: 1, y: 1 });

            assert_eq!(path.len(), 0);
        
    }
    
    #[test]
    fn pathfinding_invalid_to_invalid_test() {
        static PROJECT_DIR: Dir = include_dir!("assets");
        let lib_rs = PROJECT_DIR.get_file("tiledmap.json").unwrap();
        let body = lib_rs.contents_utf8().unwrap();
        let map_cast: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
        let pathfinder: Pathfinder = Pathfinder::new(map_cast);

        let path = pathfinder.find_path(TilePosition { x: 0, y: 1 }, TilePosition { x: 0, y: 9 });

        assert_eq!(path.len(), 0);
    }
    
    #[test]
    fn pathfinding_valid_to_invalid_test() {
        static PROJECT_DIR: Dir = include_dir!("assets");
        let lib_rs = PROJECT_DIR.get_file("tiledmap.json").unwrap();
        let body = lib_rs.contents_utf8().unwrap();
        let map_cast: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
        let pathfinder: Pathfinder = Pathfinder::new(map_cast);

        let path = pathfinder.find_path(TilePosition { x: 1, y: 1 }, TilePosition { x: 0, y: 7 });
        
        assert_eq!(path.len(), 0);
    }
    
    #[test]
    fn pathfinding_invalid_to_valid_test() {
        static PROJECT_DIR: Dir = include_dir!("assets");
        let lib_rs = PROJECT_DIR.get_file("tiledmap.json").unwrap();
        let body = lib_rs.contents_utf8().unwrap();
        let map_cast: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
        let pathfinder: Pathfinder = Pathfinder::new(map_cast);

        let path = pathfinder.find_path(TilePosition { x: 0, y: 0 }, TilePosition { x: 5, y: 5 });

        assert_eq!(path.len(), 0);
    }

    #[test]
    fn pathfinding_benchmark()
    {
        static PROJECT_DIR: Dir = include_dir!("assets");
        let lib_rs = PROJECT_DIR.get_file("tiledmap.json").unwrap();
        let body = lib_rs.contents_utf8().unwrap();
        let map_cast: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
        let pathfinder: Pathfinder = Pathfinder::new(map_cast);

        let duration = bench_find_path(pathfinder);
        println!("Duration:{}ms", duration.as_millis());
        assert!(duration.as_millis()< 10);
    }
    
    fn bench_find_path(pathfinder: Pathfinder) -> Duration{
        let start = Instant::now();
        pathfinder.find_path(TilePosition { x: 1, y: 1 }, TilePosition { x: 45, y: 45 });
        start.elapsed()
        
    }
}
