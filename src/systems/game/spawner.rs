use std::time::Duration;

use bevy::prelude::*;
use derive_builder::Builder;
use uuid::Uuid;

use super::Product;

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

/// Tags a [Product] entity has originating from a [Spawner] with the same id
#[derive(Component)]
pub struct SpawnerTag(pub Uuid);

fn tick_spawners(
  mut commands: Commands,
  time: Res<Time>,
  mut spawners: Query<(&mut Spawner, &Transform)>,
  spawner_objs: Query<&SpawnerTag, With<Product>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(tick_spawners);
  }
}
