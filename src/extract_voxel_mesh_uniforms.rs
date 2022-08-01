use super::voxel;

use bevy::{
    pbr::{MeshUniform, NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

bitflags::bitflags! {
    #[repr(transparent)]
    struct MeshFlags: u32 {
        const SHADOW_RECEIVER            = (1 << 0);
        const NONE                       = 0;
        const UNINITIALIZED              = 0xFFFF;
    }
}

pub fn extract_voxel_mesh_uniforms(
    mut commands: Commands,
    mut previous_caster_len: Local<usize>,
    mut previous_not_caster_len: Local<usize>,
    caster_query: Query<
        (
            Entity,
            &ComputedVisibility,
            &GlobalTransform,
            Option<&NotShadowReceiver>,
        ),
        (With<voxel::Voxel>, Without<NotShadowCaster>),
    >,
    not_caster_query: Query<
        (
            Entity,
            &ComputedVisibility,
            &GlobalTransform,
            Option<&NotShadowReceiver>,
        ),
        (With<voxel::Voxel>, With<NotShadowCaster>),
    >,
) {
    let mut caster_values = Vec::with_capacity(*previous_caster_len);
    for (entity, computed_visibility, transform, not_receiver) in caster_query.iter() {
        if !computed_visibility.is_visible {
            continue;
        }
        let transform = transform.compute_matrix();
        caster_values.push((
            entity,
            (MeshUniform {
                flags: if not_receiver.is_some() {
                    MeshFlags::empty().bits
                } else {
                    MeshFlags::SHADOW_RECEIVER.bits
                },
                transform,
                inverse_transpose_model: transform.inverse().transpose(),
            },),
        ));
    }
    *previous_caster_len = caster_values.len();
    commands.insert_or_spawn_batch(caster_values);

    let mut not_caster_values = Vec::with_capacity(*previous_not_caster_len);
    for (entity, computed_visibility, transform, not_receiver) in not_caster_query.iter() {
        if !computed_visibility.is_visible {
            continue;
        }
        let transform = transform.compute_matrix();
        not_caster_values.push((
            entity,
            (
                MeshUniform {
                    flags: if not_receiver.is_some() {
                        MeshFlags::empty().bits
                    } else {
                        MeshFlags::SHADOW_RECEIVER.bits
                    },
                    transform,
                    inverse_transpose_model: transform.inverse().transpose(),
                },
                NotShadowCaster,
            ),
        ));
    }
    *previous_not_caster_len = not_caster_values.len();
    commands.insert_or_spawn_batch(not_caster_values);
}
