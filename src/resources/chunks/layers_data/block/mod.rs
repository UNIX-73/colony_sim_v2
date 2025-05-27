pub mod material_resource;

use super::OcclussionCulling;
use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{Display, EnumCount, EnumIter};

#[repr(u16)]
#[derive(PartialEq, Debug, Eq, Hash, Clone, Copy, Display, EnumCount, EnumIter)]
pub enum Block {
    Air,
    Water,
    Dirt,
    Granite,
    // Más en un futuro
}
impl Default for Block {
    fn default() -> Self {
        Self::Air
    }
}
impl Block {
    pub fn get_color(&self) -> Option<(u8, u8, u8)> {
        match self {
            Block::Air => None,
            Block::Water => Some((0, 119, 190)),     // Azul agua
            Block::Dirt => Some((134, 83, 41)),      // Marrón tierra
            Block::Granite => Some((112, 128, 144)), // Gris granito
        }
    }

    pub fn from_u16(idx: u16) -> Block {
        if idx < Block::COUNT as u16 {
            Block::iter().nth(idx as usize).unwrap()
        } else {
            Block::Air
        }
    }
}
impl OcclussionCulling for Block {
    fn occludes(&self) -> bool {
        match self {
            Block::Air => false,
            _ => true,
        }
    }
}
