use bevy::prelude::*;
use bevy_attention_tracker::{Attent, AttentionTracker};
use smooth_bevy_cameras::{controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin}, LookTransformPlugin};

fn main() {
  App::new()
    .add_plugins((DefaultPlugins, AttentionTracker, LookTransformPlugin, FpsCameraPlugin::default()))
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
  // Spawn the camera
  commands.spawn((Camera3dBundle::default(), Attent)).insert(FpsCameraBundle::new(
    FpsCameraController::default(), Vec3::new(2., 1., 2.), Vec3::ZERO, Vec3::Y)
  );

  // Spawn the central cube object
  commands.spawn(PbrBundle {
    transform: Transform::from_xyz(0., 0.5, 0.),
    mesh: meshes.add(Cuboid::default()),
    material: materials.add(Color::ALICE_BLUE),
    ..default()
  });

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