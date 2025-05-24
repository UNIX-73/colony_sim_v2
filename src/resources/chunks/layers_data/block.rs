use strum_macros::{Display, EnumCount};

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
        Self::Air
    }
}
impl SurfaceBlock {
    pub fn get_color(&self) -> Option<(u8, u8, u8)> {
        match self {
            SurfaceBlock::Air => None,
            SurfaceBlock::Water => Some((0, 119, 190)), // Azul agua
            SurfaceBlock::Dirt => Some((134, 83, 41)),  // Marrón tierra
            SurfaceBlock::Granite => Some((112, 128, 144)), // Gris granito
        }
    }
}
