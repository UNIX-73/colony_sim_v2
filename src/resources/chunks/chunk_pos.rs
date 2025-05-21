#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}
impl ChunkPos {
    pub fn new(x: i32, y: i32) -> ChunkPos {
        ChunkPos { x, y }
    }
}
