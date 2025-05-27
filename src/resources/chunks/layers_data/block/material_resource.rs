use std::array;

use bevy::prelude::*;
use strum::EnumCount;

use super::Block;

#[derive(Resource)]
pub struct BlockInstancing {
    pub materials: [Option<Handle<StandardMaterial>>; Block::COUNT],
    pub mesh: Handle<Mesh>,
}
impl BlockInstancing {
    pub fn init(
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) -> BlockInstancing {
        let materials: [Option<Handle<StandardMaterial>>; Block::COUNT] = array::from_fn(|i| {
            let color = Block::from_u16(i as u16).get_color();

            if let Some(color) = color {
                Some(materials.add(Color::srgb_u8(color.0, color.1, color.2)))
            } else {
                None
            }
        });

        let mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

        BlockInstancing { materials, mesh }
    }

    #[inline]
    pub fn get_components(&self, block: Block) -> Option<(Handle<StandardMaterial>, Handle<Mesh>)> {
        let material = self.materials[block as usize].clone();

        if let Some(material) = material {
            Some((material, self.mesh.clone()))
        } else {
            None
        }
    }
}
