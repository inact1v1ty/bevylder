use bevy::{prelude::*, render::extract_resource::ExtractResourcePlugin};

mod common;
mod entity;
pub mod wireframe;
mod world;

pub use common::{VoxelData, VoxelMesh};

pub use entity::EntityVoxelPlugin;

pub use entity::bundle::VoxelBundle as EntityVoxelBundle;
pub use entity::voxel::Voxel as EntityVoxel;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ExtractResourcePlugin::<VoxelMesh>::default())
            .init_resource::<VoxelMesh>();

        app.add_plugin(EntityVoxelPlugin);
    }
}
