use serde::{Deserialize, Serialize};
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
