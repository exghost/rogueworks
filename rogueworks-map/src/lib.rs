//! Map data structures and generators (stubs).

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TileKind { Floor, Wall }

bitflags::bitflags! {
    pub struct TileFlags: u32 {
        const BLOCKS_MOTION = 0b0001;
        const BLOCKS_SIGHT  = 0b0010;
    }
}

#[derive(Debug, Clone)]
pub struct Tile { pub kind: TileKind, pub flags: TileFlags }

#[derive(Debug, Clone)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
}

impl Map {
    pub fn new(width: i32, height: i32, fill: Tile) -> Self {
        Self { width, height, tiles: vec![fill; (width*height) as usize] }
    }
    pub fn idx(&self, x: i32, y: i32) -> usize { (y * self.width + x) as usize }
}
