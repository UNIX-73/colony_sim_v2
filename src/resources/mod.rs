pub mod chunks;
pub mod render;
pub mod simulation;

use crate::utils::multithread::mutex::Mtx;
use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct SharedThreadResource<T> {
    pub resource: Mtx<T>,
}
impl<T> SharedThreadResource<T> {
    pub fn new(resource: Mtx<T>) -> SharedThreadResource<T> {
        SharedThreadResource { resource }
    }

    pub fn from_arc(resource: Mtx<T>) -> Self {
        SharedThreadResource { resource }
    }

    pub fn get(&self) -> &Mtx<T> {
        &self.resource
    }
}
