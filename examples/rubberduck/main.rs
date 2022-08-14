use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::event::Events,
    prelude::*,
    window::WindowFocused,
};
use bevy_clap::ClapPlugin;
use clap;

use bevylder::{
    wireframe::{VoxelWireframeConfig, VoxelWireframePlugin},
    Voxel, VoxelBundle, VoxelData, VoxelPlugin,
};
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

mod bevy_clap;
mod test_model;

#[derive(clap::Parser)]
#[clap()]
struct Context {
    #[clap(long, default_missing_value("20"))]
    stress: Option<i32>,
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(WindowDescriptor {
            title: "bevylder rubberduck".to_string(),
            cursor_locked: true,
            cursor_visible: false,
            ..Default::default()
        })
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .add_plugin(FpsCameraPlugin::default())
        .add_plugin(VoxelPlugin)
        .insert_resource(VoxelWireframeConfig { global: true })
        .add_plugin(VoxelWireframePlugin)
        .add_plugin(ClapPlugin::<Context>::default())
        .add_system(bevy::window::close_on_esc)
        .add_system(disable_on_blur)
        .add_startup_system(setup)
        .add_system(fps_system)
        .run();
}

#[derive(Component)]
struct StatsText;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut voxel_data: ResMut<Assets<VoxelData>>,
    flags: Res<Context>,
    asset_server: Res<AssetServer>,
) {
    if let Some(num) = flags.stress {
        for x in -num..=num {
            for z in -num..=num {
                commands.spawn_bundle(VoxelBundle {
                    voxel: Voxel {
                        data: voxel_data.add(VoxelData(test_model::TEST_MODEL_DUCK)),
                    },
                    transform: Transform::from_xyz(2.0 * x as f32, 0.0, 2.0 * z as f32),
                    ..default()
                });
            }
        }

        commands
            .spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "Block Count: ".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                font_size: 20.0,
                                color: Color::rgb(0.0, 1.0, 0.0),
                            },
                        },
                        TextSection {
                            value: ((num * 2 + 1) * (num * 2 + 1)).to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                font_size: 20.0,
                                color: Color::rgb(0.0, 1.0, 1.0),
                            },
                        },
                        TextSection {
                            value: "\nAverage FPS: ".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                font_size: 20.0,
                                color: Color::rgb(0.0, 1.0, 0.0),
                            },
                        },
                        TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                font_size: 20.0,
                                color: Color::rgb(0.0, 1.0, 1.0),
                            },
                        },
                    ],
                    ..default()
                },
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(5.0),
                        left: Val::Px(5.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            })
            .insert(StatsText);
    } else {
        commands.spawn_bundle(VoxelBundle {
            voxel: Voxel {
                data: voxel_data.add(VoxelData(test_model::TEST_MODEL_DUCK)),
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        });

        commands.spawn_bundle(VoxelBundle {
            voxel: Voxel {
                data: voxel_data.add(VoxelData(test_model::TEST_MODEL_DUCK)),
            },
            transform: Transform::from_xyz(2.0, 0.0, 0.0),
            ..default()
        });
    }

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands
        .spawn_bundle(Camera3dBundle::default())
        .insert_bundle(FpsCameraBundle::new(
            FpsCameraController {
                translate_sensitivity: 0.1,
                mouse_rotate_sensitivity: Vec2::splat(0.001),
                ..Default::default()
            },
            Vec3::new(-1.0, 1.25, 2.5),
            Vec3::ZERO,
        ));
}

fn disable_on_blur(
    mut windows: ResMut<Windows>,
    focused_event: Res<Events<WindowFocused>>,
    mut controllers: Query<&mut FpsCameraController>,
) {
    if let Some(window) = windows.get_primary_mut() {
        let focus_changed = focused_event
            .get_reader()
            .iter(&focused_event)
            .any(|ev| ev.id == window.id());

        if focus_changed {
            let mut controller = controllers.single_mut();
            controller.enabled = window.is_focused();

            window.set_cursor_visibility(!window.is_focused());
            window.set_cursor_lock_mode(window.is_focused());
        }
    }
}

fn fps_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<StatsText>>) {
    if let Ok(mut text) = query.get_single_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[3].value = format!("{:.2}", average);
            }
        };
    }
}
