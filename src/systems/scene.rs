use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::game::{Product, ProductBuilder, SpawnerBuilder};

pub fn setup_graphics(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(0.0, 30.0, 150.0).looking_at(Vec3::new(0.0, 30.0, 0.0), Vec3::Y),
    ..Default::default()
  });

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

  commands
    .spawn(PbrBundle {
      mesh: meshes.add(shape::Box::new(50.0, 1.0, 50.0).into()),
      material: materials.add(Color::WHITE.into()),
      transform: Transform::from_xyz(0.0, -10.0, 0.0).with_rotation(Quat::from_axis_angle(Vec3::Z, 85.0)),
      ..Default::default()
    })
    .insert(RigidBody::Fixed)
    .insert(Collider::cuboid(25.0, 0.5, 25.0));

  commands
    .spawn(PbrBundle {
      mesh: meshes.add(shape::Cube::new(4.0).into()),
      material: materials.add(Color::BLUE.into()),
      transform: Transform::from_xyz(0.0, 10.0, 5.0),
      ..Default::default()
    })
    .insert(SpawnerBuilder::default().build().unwrap());
}
