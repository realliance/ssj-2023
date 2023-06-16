use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{Consumer, GameState, Product};

fn display_events(
  mut commands: Commands,
  mut state: ResMut<GameState>,
  mut contact_force_events: EventReader<ContactForceEvent>,
  is_consumer_sensor: Query<Entity, With<Consumer>>,
  mut product_query: Query<&mut Product>,
) {
  for event in contact_force_events.iter() {
    // Find product

    let is_coll_1_product = product_query.get(event.collider1).is_ok();
    let is_coll_2_product = product_query.get(event.collider2).is_ok();

    let product_ent;

    let mut product = if is_coll_1_product {
      product_ent = event.collider1;
      product_query.get_mut(event.collider1).unwrap()
    } else if is_coll_2_product {
      product_ent = event.collider2;
      product_query.get_mut(event.collider2).unwrap()
    } else {
      // Exit, nothing to do here
      return;
    };

    product.add_hit_mult(event.max_force_magnitude);

    // Check for consumer sensor
    let sensor = is_consumer_sensor
      .get(event.collider1)
      .or_else(|_| is_consumer_sensor.get(event.collider2));

    if sensor.is_ok() {
      state.currency += product.payment();
      commands.entity(product_ent).despawn();
    }
  }
}

fn configure_rapier(mut rapier_config: ResMut<RapierConfiguration>) {
  rapier_config.timestep_mode = TimestepMode::Interpolated {
    dt: 1.0 / 60.0,
    time_scale: 1.0,
    substeps: 1,
  }
}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(configure_rapier).add_system(display_events);
  }
}
