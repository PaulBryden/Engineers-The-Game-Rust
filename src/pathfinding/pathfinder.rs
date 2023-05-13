use super::super::tiledmap;
use macroquad::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};

// Pathfinder struct that contains a tilemap
pub struct Pathfinder {
    tilemap: tiledmap::TiledMap,
}

// TilePosition struct that represents the position of a tile on the map
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
}

// State struct that represents the cost and position of a tile
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    cost: i32,
    position: TilePosition,
}

// Implement Ord trait for State to allow it to be used in a BinaryHeap
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// Implement PartialOrd trait for State to allow it to be used in a BinaryHeap
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Heuristic function that calculates the Manhattan distance between two points
pub fn heuristic(a: &TilePosition, b: &TilePosition) -> i32 {
    (b.x - a.x).abs() + (b.y - a.y).abs()
}

// TilePositionKeyEntry struct that represents the position and key of a tile
#[derive(Debug, Clone)]
pub struct TilePositionKeyEntry {
    position: TilePosition,
    key: String,
}

// Function that converts a tile position to a string key
pub fn tile_position_to_key(x: i32, y: i32) -> String {
    [x.to_string(), y.to_string()].join("x")
}

impl Pathfinder {
    // Constructor for Pathfinder
    pub fn new(map: tiledmap::TiledMap) -> Pathfinder {
        Pathfinder { tilemap: map }
    }

    // Method that checks if a tile is walkable
    pub fn tile_is_walkable(&self, x: i32, y: i32) -> bool {
        if x >= 0 && y >= 0 {
            //remember to decrement the tile number.
            !self.tilemap.tilesets[0].tiles[self.tilemap.layers[0].get_tile_at(x as u32, y as u32) as usize - 1 as usize]
                .properties[0]
                .value
        } else {
            return false;
        }
    }

    // Method that returns the neighbours of a given tile position
    pub fn get_neighbours(&self, a: &TilePosition) -> Vec<TilePosition> {
        let mut neighbours = Vec::new();
        neighbours.push(TilePosition { x: a.x - 1, y: a.y - 1 });
        neighbours.push(TilePosition { x: a.x - 1, y: a.y });
        neighbours.push(TilePosition { x: a.x - 1, y: a.y + 1 });
        neighbours.push(TilePosition { x: a.x, y: a.y - 1 });
        neighbours.push(TilePosition { x: a.x, y: a.y });
        neighbours.push(TilePosition { x: a.x, y: a.y + 1 });
        neighbours.push(TilePosition { x: a.x + 1, y: a.y - 1 });
        neighbours.push(TilePosition { x: a.x + 1, y: a.y });
        neighbours.push(TilePosition { x: a.x + 1, y: a.y + 1 });
        neighbours
    }

    // Heuristic function that calculates the Manhattan distance between two points
    fn heuristic(a: &TilePosition, b:&TilePosition) -> i32{
        (b.x-a.x).abs()+(b.y-a.y).abs()
    }

    // Method that finds the shortest path between two tiles using the A* algorithm
    pub fn find_path(&self,start :TilePosition,target :TilePosition)->Vec<TilePosition>{
        let mut path :Vec<TilePosition>=Vec::new();
        if !self.tile_is_walkable(target.x,target.y){
            return path;
        }
        if !self.tile_is_walkable(start.x,start.y){
            return path;
        }

        let mut heap = BinaryHeap::new();
        heap.push(State{cost :0 ,position:start});

    let mut parent_for_key: HashMap<String, TilePositionKeyEntry> = HashMap::new();
    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(start, 0);

    let start_key = tile_position_to_key(start.x, start.y);
    let target_key = tile_position_to_key(target.x, target.y);
    parent_for_key.insert(
        start_key.clone(),
        TilePositionKeyEntry {
            key: "".to_string(),
            position: TilePosition { x: 0, y: 0 },
        },
    );

    while let Some(State { cost, position }) = heap.pop() {
        if position == target {
            break;
        }

        let current_key = tile_position_to_key(position.x, position.y);
        let neighbours = self.get_neighbours(&position);

        for neighbour in neighbours {
            if !self.tile_is_walkable(neighbour.x, neighbour.y) {
                continue;
            }
            let new_cost = cost_so_far[&position] + 1;
            if !cost_so_far.contains_key(&neighbour) || new_cost < cost_so_far[&neighbour] {
                cost_so_far.insert(neighbour, new_cost);
                let priority = new_cost + heuristic(&neighbour, &target);
                heap.push(State {
                    cost: priority,
                    position: neighbour,
                });
                parent_for_key.insert(
                    tile_position_to_key(neighbour.x, neighbour.y),
                    TilePositionKeyEntry {
                        key: current_key.clone(),
                        position: neighbour,
                    },
                );
            }
        }
    }

    let mut current_key = target_key;
    let mut current_pos;

    while current_key != start_key {
        let tile_pos_key_entry: TilePositionKeyEntry = parent_for_key[&current_key].clone();
        current_key = tile_pos_key_entry.key;
        current_pos = tile_pos_key_entry.position;
        path.push(std::mem::take(&mut current_pos));
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
    use std::time::Duration;
    use std::time::Instant;
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
    fn pathfinding_benchmark() {
        static PROJECT_DIR: Dir = include_dir!("assets");
        let lib_rs = PROJECT_DIR.get_file("tiledmap.json").unwrap();
        let body = lib_rs.contents_utf8().unwrap();
        let map_cast: tiledmap::TiledMap = serde_json::from_str(&body).unwrap();
        let pathfinder: Pathfinder = Pathfinder::new(map_cast);

        let duration = bench_find_path(pathfinder);
        println!("Duration:{}ms", duration.as_millis());
        assert!(duration.as_millis() < 10);
    }

    fn bench_find_path(pathfinder: Pathfinder) -> Duration {
        let start = Instant::now();
        pathfinder.find_path(TilePosition { x: 1, y: 1 }, TilePosition { x: 45, y: 45 });
        start.elapsed()
    }
}
