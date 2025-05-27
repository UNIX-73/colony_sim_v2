pub mod block;
pub trait CellData: Clone + PartialEq + Default + OcclussionCulling {}
impl<T: Clone + PartialEq + Default + OcclussionCulling> CellData for T {}

pub trait OcclussionCulling {
    fn occludes(&self) -> bool;
}
