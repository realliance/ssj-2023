use super::ProductShape;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Clone, Component)]
pub enum MutatorScreen {
  Bounce(f32),
  Mass(f32),
  Friction(f32),
  Shape(ProductShape),
}

impl MutatorScreen {
  pub fn color(&self) -> Color {
    match self {
      MutatorScreen::Bounce(_) => Color::AQUAMARINE,
      MutatorScreen::Mass(_) => Color::DARK_GRAY,
      MutatorScreen::Friction(_) => Color::CRIMSON,
      MutatorScreen::Shape(_) => Color::PURPLE,
    }
    .with_a(0.8)
  }

  pub fn into_entity<V: Into<Vec3> + Clone>(
    self,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: V,
  ) -> Entity {
    commands
      .spawn(self.clone())
      .insert(PbrBundle {
        mesh: meshes.add(shape::Box::new(8.0, 1.0, 8.0).into()),
        material: materials.add(self.color().into()),
        transform: Transform::from_translation(position.into()),
        ..Default::default()
      })
      .insert(Collider::cuboid(4.0, 0.5, 4.0))
      .insert(Sensor)
      .insert(bevy_mod_picking::PickableBundle::default())
      .insert(bevy_transform_gizmo::GizmoTransformable)
      .id()
  }
}
