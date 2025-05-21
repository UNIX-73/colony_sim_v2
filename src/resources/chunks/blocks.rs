#[repr(u16)]
#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy, Display, EnumCount)]
pub enum SurfaceBlock {
    Air,
    Water,
    Dirt,
    Granite,
    // Más en un futuro
}
impl Default for SurfaceBlock {
    fn default() -> Self {
        Self::Dirt
    }
}
