use bevy::prelude::*;
use bevy_attention_tracker::{
    observation::ObjectObservation, observer::ProximityObserver, visualizer::DebugVisualizerPlugin,
    AttentionTrackerPlugin,
};
use bevy_third_person_camera::{
    camera::Zoom, controller::ThirdPersonController, ThirdPersonCamera, ThirdPersonCameraPlugin,
    ThirdPersonCameraTarget,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            AttentionTrackerPlugin,
            DebugVisualizerPlugin,
            ThirdPersonCameraPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn the camera
    commands.spawn((
        Camera3dBundle::default(),
        ThirdPersonCamera {
            zoom: Zoom::new(1., 20.),
            ..default()
        },
    ));

    // Spawn the player avatar
    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(-1., 0.5, 0.).with_scale(Vec3::splat(0.5)),
            mesh: meshes.add(Capsule3d::default()),
            material: materials.add(Color::PINK),
            ..default()
        },
        ThirdPersonCameraTarget,
        ThirdPersonController::default(),
        ProximityObserver::new(1.),
    ));

    // Spawn the central cube object
    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(0., 0.5, 0.),
            mesh: meshes.add(Cuboid::default()),
            material: materials.add(Color::ALICE_BLUE),
            ..default()
        },
        ObjectObservation::default(),
        Name::new("Central Cube"),
    ));

    // Spawn the ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(10., 10.)),
        material: materials.add(Color::YELLOW_GREEN),
        ..default()
    });

    // Spawn the directional light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(-1., 4., 1.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
