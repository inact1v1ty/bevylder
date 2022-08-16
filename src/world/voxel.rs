use bevy::prelude::*;

use crate::common::VoxelData;

#[derive(Component, Clone, Default)]
pub struct Voxel {
    pub data: Handle<VoxelData>,
}
