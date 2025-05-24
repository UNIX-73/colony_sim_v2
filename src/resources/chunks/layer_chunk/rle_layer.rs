use super::{LayerChunk, chunk_data::chunk_cell_pos::ChunkCellPos};
use crate::resources::chunks::{CHUNK_VOLUME, CellData, layer_chunk::chunk_data::ChunkData};
use crate::utils::memory_size::MemorySize;
use std::mem::MaybeUninit;
use std::mem::size_of;

pub struct RleRun<T: CellData> {
    pub id: T,
    pub count: u16,
}

pub struct RleChunk<T: CellData>(Vec<RleRun<T>>);
impl<T: CellData> LayerChunk<T> for RleChunk<T> {
    /// INEFICIENTE recorre todo el vec en busca de un bloque por lo que es O(n)
    #[inline]
    fn get_index(&self, idx: usize) -> T {
        debug_assert!(
            idx < CHUNK_VOLUME,
            "Requested cell out of bounds of chunk by idx {}",
            idx
        );

        let mut i = 0usize;
        for run in &self.0 {
            if idx < i + run.count as usize {
                return run.id.clone();
            }
            i += run.count as usize;
        }

        panic!("RLE decoding failed: index {} not found in any run", idx);
    }

    #[inline]
    fn get_pos(&self, pos: ChunkCellPos) -> T {
        self.get_index(pos.idx())
    }

    #[inline]
    fn get_xyz(&self, x: usize, y: usize, z: usize) -> T {
        self.get_index(ChunkCellPos::idx_from_xyz(x, y, z))
    }

    fn unzip(&self) -> ChunkData<T> {
        debug_assert!(self.is_valid());

        let mut raw: [MaybeUninit<T>; CHUNK_VOLUME] =
            unsafe { MaybeUninit::uninit().assume_init() };

        let mut idx = 0_usize;
        for run in &self.0 {
            for _ in 0..run.count {
                debug_assert!(
                    idx < CHUNK_VOLUME,
                    "Index {} out of bounds during unzip",
                    idx
                );
                raw[idx].write(run.id.clone());
                idx += 1;
            }
        }

        if idx != CHUNK_VOLUME {
            panic!(
                "RLE chunk size is {:0} when it should be {:1}",
                idx, CHUNK_VOLUME
            )
        }

        ChunkData::new(std::array::from_fn(|i| unsafe {
            raw[i].assume_init_read()
        }))
    }

    

    fn from_unzip(unzip: super::chunk_data::ChunkData<T>) -> Self {
        debug_assert!(
            CHUNK_VOLUME <= u16::MAX as usize,
            "RLE run count would overflow u16"
        );

        let mut data: Vec<RleRun<T>> = Vec::new();

        let mut iter = unzip.data().into_iter();
        let mut last_cell = iter.next().unwrap(); // asumimos que CHUNK_VOLUME > 0
        let mut count = 1;

        for cell in iter {
            if cell == last_cell {
                count += 1;
            } else {
                data.push(RleRun {
                    id: last_cell.clone(),
                    count,
                });
                last_cell = cell;
                count = 1;
            }
        }

        data.push(RleRun {
            id: last_cell.clone(),
            count,
        });

        Self(data)
    }

    fn memory_usage(&self) -> crate::utils::memory_size::MemorySize {
        let run_size = size_of::<RleRun<T>>();
        let vec_len = self.0.len();
        let vec_base = size_of::<Vec<RleRun<T>>>(); // típicamente 24 bytes

        let total = vec_base + vec_len * run_size;

        let bytes = total % (1024 * 1024);

        MemorySize::new(bytes as u64)
    }

    fn is_valid(&self) -> bool {
        debug_assert!(
            CHUNK_VOLUME <= u16::MAX as usize,
            "RLE run count would overflow u16"
        );

        let mut volume = 0;
        for run in &self.0 {
            volume += run.count
        }

        volume as usize == CHUNK_VOLUME
    }

    fn iter(&self) -> impl Iterator<Item = (usize, T)> + '_ {
        let mut index = 0;
        self.0.iter().flat_map(move |run| {
            let start = index;
            index += run.count as usize;
            (start..index).map(move |i| (i, run.id.clone()))
        })
    }
}
