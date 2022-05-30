use crate::prelude::*;

pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
            TileType::Exit => to_cp437('>'),
        }
    }
}

pub struct ForestTheme {}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(';'),
            TileType::Wall => to_cp437('"'),
            TileType::Exit => to_cp437('>'),
        }
    }
}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

pub struct LavaTheme {}

impl MapTheme for LavaTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('1'),
            TileType::Wall => to_cp437('2'),
            TileType::Exit => to_cp437('>'),
        }
    }
}

impl LavaTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}
pub struct WaterTheme {}

impl MapTheme for WaterTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('4'),
            TileType::Wall => to_cp437('3'),
            TileType::Exit => to_cp437('>'),
        }
    }
}

impl WaterTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}
pub struct JungleTheme {}

impl MapTheme for JungleTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('6'),
            TileType::Wall => to_cp437('5'),
            TileType::Exit => to_cp437('>'),
        }
    }
}

impl JungleTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}
