use serde::{Deserialize, Serialize};
use super::sprites::tilesprite::TileSprite;
use macroquad::{texture::Texture2D, rand};
use super::sprites::sprite::{SpriteID};

#[derive(Serialize, Deserialize, Debug)]
pub struct TiledMap {
    compressionlevel: i64,
    editorsettings: EditorSettings,
    pub height: i64,
    infinite: bool,
    pub layers: Vec<LayerData>,
    nextlayerid: i64,
    nextobjectid: i64,
    orientation: String,
    renderorder: String,
    tiledversion: String,
    tileheight: i64,
    pub tilesets: Vec<TileSet>,
    tilewidth: i64,
    #[serde(rename = "type")]
    tiled_type: String,
    version: f64,
    pub width: i64,
}
#[derive(Serialize, Deserialize, Debug)]
struct EditorSettings {
    export: Export,
}
#[derive(Serialize, Deserialize, Debug)]
struct Export {
    format: String,
    target: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LayerData {
    pub data: Vec<i64>,
    pub height: i64,
    pub id: i64,
    name: String,
    opacity: i64,
    #[serde(rename = "type")]
    tiled_type: String,
    visible: bool,
    pub width: i64,
    x: i64,
    y: i64,
}

impl LayerData
{
    pub fn get_tile_at(&self, x: u32, y: u32) -> u32
    {
        self.data[((y*(self.width as u32))+x) as usize] as u32
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TileSet {
    columns: i64,
    firstgid: i64,
    image: String,
    imageheight: i64,
    imagewidth: i64,
    margin: i64,
    name: String,
    spacing: i64,
    tilecount: i64,
    tileheight: i64,
    #[serde(default)]
    pub tiles: Vec<Tile>,
    tilewidth: i64,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Tile {
    pub id: i64,
    pub properties: Vec<Property>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Property {
    name: String,
    #[serde(rename = "type")]
    tiled_type: String,
    pub value: bool,
}

pub fn get_tilemap_spritelist(tileset: Texture2D, tilemap: &TiledMap) -> std::collections::HashMap<u32,SpriteID> {
    let mut sprite_store: std::collections::HashMap<u32,SpriteID> = std::collections::HashMap::new();
    for layer_num in 0 as usize..tilemap.layers.len() as usize {
        for y in 0 as u32..tilemap.height as u32 {
            for x in 0 as u32..tilemap.width as u32 {
                let uuid: u32 = rand::rand();
                let tile_sprite: SpriteID = SpriteID::Tile(TileSprite {
                    layer: layer_num as u32,
                    x: x,
                    y: y,
                    texture: tileset,
                    frame_number: tilemap.layers[layer_num].data
                        [(y * tilemap.layers[layer_num].width as u32 + x) as usize]
                        as u32
                        - 1,
                    width: 64.0,
                    height: 32.0,
                    uuid: uuid,
                });
                sprite_store.insert(uuid, tile_sprite);
            }
        }
    }
    sprite_store
}
