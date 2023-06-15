use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Consumers consume product and give the player money based on the product's properties.
#[derive(Component)]
pub struct Consumer;

const SQUARE_SIZE: f32 = 5.0;

impl Consumer {
  pub fn into_entity<V: Into<Vec3> + Clone>(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: V,
  ) -> Entity {
    commands
      .spawn(PbrBundle {
        mesh: meshes.add(shape::Box::new(SQUARE_SIZE, 0.5, SQUARE_SIZE).into()),
        material: materials.add(Color::GOLD.into()),
        transform: Transform::from_translation(position.into()),
        ..Default::default()
      })
      .insert(Self)
      .insert(RigidBody::Fixed)
      .insert(Collider::cuboid(SQUARE_SIZE / 2.0, 0.25, SQUARE_SIZE / 2.0))
      .insert(bevy_mod_picking::PickableBundle::default())
      .insert(bevy_transform_gizmo::GizmoTransformable)
      .id()
  }
}
