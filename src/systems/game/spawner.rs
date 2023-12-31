use std::time::Duration;

use bevy::prelude::*;
use derive_builder::Builder;
use uuid::Uuid;

use super::{GameState, MarkForDespawn, Product};

/// Produces [Product]
#[derive(Component, Builder)]
#[builder(default)]
pub struct Spawner {
  pub id: Uuid,
  pub cooldown_timer: Timer,
  pub max_spawned: usize,
  pub product_template: Product,
}

impl Default for Spawner {
  fn default() -> Self {
    Self {
      id: Uuid::new_v4(),
      cooldown_timer: Timer::new(Duration::from_millis(850), TimerMode::Repeating),
      max_spawned: 3,
      product_template: Default::default(),
    }
  }
}

impl Spawner {
  pub fn into_entity<V: Into<Vec3> + Clone>(
    self,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: V,
  ) -> Entity {
    commands
      .spawn(PbrBundle {
        mesh: meshes.add(shape::Cube::new(4.0).into()),
        material: materials.add(Color::BLUE.into()),
        transform: Transform::from_translation(position.into()),
        ..Default::default()
      })
      .insert(self)
      .insert(bevy_mod_picking::PickableBundle::default())
      .insert(bevy_transform_gizmo::GizmoTransformable)
      .id()
  }
}

/// Tags a [Product] entity has originating from a [Spawner] with the same id
#[derive(Component)]
pub struct SpawnerTag(pub Uuid);

fn tick_spawners(
  mut commands: Commands,
  time: Res<Time>,
  state: Res<GameState>,
  mut spawners: Query<(&mut Spawner, &Transform)>,
  spawner_objs: Query<&SpawnerTag, With<Product>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  if !state.run {
    return;
  }

  let dt = time.delta();
  spawners.for_each_mut(|(mut spawner, trans)| {
    if spawner.cooldown_timer.tick(dt).just_finished() {
      // TODO linear operation per timer finish is not ideal
      let obj_count = spawner_objs.iter().filter(|x| x.0 == spawner.id).count();
      if obj_count < spawner.max_spawned {
        let product = spawner.product_template.clone().into_entity(
          &mut commands,
          &mut meshes,
          &mut materials,
          trans.translation.clone(),
        );

        commands.entity(product).insert(SpawnerTag(spawner.id.clone()));
      }
    }
  });
}

#[derive(Default)]
pub struct ClearProducts;

fn clear_products(
  mut commands: Commands,
  active_products: Query<Entity, With<Product>>,
  mut event: EventReader<ClearProducts>,
) {
  for _ev in event.iter() {
    active_products.for_each(|e| {
      commands.entity(e).insert(MarkForDespawn);
    });
  }
}

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<ClearProducts>()
      .add_system(tick_spawners)
      .add_system(clear_products);
  }
}
