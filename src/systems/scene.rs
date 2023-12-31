use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::controllers::fps::{FpsCameraBundle, FpsCameraController};

use super::game::{Consumer, MutatorScreen, ProductShape, SpawnerBuilder};

pub fn setup_graphics(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands
    .spawn(Camera3dBundle::default())
    .insert(bevy_mod_picking::PickingCameraBundle::default())
    .insert(bevy_transform_gizmo::GizmoPickSource::default())
    .insert(FpsCameraBundle::new(
      FpsCameraController {
        translate_sensitivity: 50.0,
        mouse_rotate_sensitivity: Vec2::splat(0.3),
        ..Default::default()
      },
      Vec3::new(10.0, 30.0, 50.0),
      Vec3::new(0., 0., 0.),
      Vec3::Y,
    ));

  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 9000.0,
      range: 100.,
      shadows_enabled: true,
      ..Default::default()
    },
    transform: Transform::from_xyz(8.0, 16.0, 8.0),
    ..Default::default()
  });

  SpawnerBuilder::default()
    .build()
    .unwrap()
    .into_entity(&mut commands, &mut meshes, &mut materials, [0.0, 25.0, 5.0]);

  Consumer::into_entity(&mut commands, &mut meshes, &mut materials, [0.0, 0.0, 5.0]);
}
