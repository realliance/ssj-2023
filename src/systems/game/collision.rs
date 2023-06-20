use bevy::prelude::*;
use bevy_rapier3d::{prelude::*, rapier::prelude::CollisionEventFlags};

use super::{Consumer, GameState, MutatorScreen, Product};

#[derive(Component)]
pub struct MarkForDespawn;

fn contact_force_events(
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
      continue;
    };

    product.add_hit_mult(event.max_force_magnitude);

    // Check for consumer sensor
    let sensor = is_consumer_sensor
      .get(event.collider1)
      .or_else(|_| is_consumer_sensor.get(event.collider2));

    if sensor.is_ok() {
      state.currency += product.payment();
      commands.entity(product_ent).insert(MarkForDespawn);
    }
  }
}

fn collision_events(
  mut collision_events: EventReader<CollisionEvent>,
  state: Res<GameState>,
  mut product_query: Query<(
    &mut Product,
    &mut Restitution,
    &mut Friction,
    &mut ColliderMassProperties,
    &mut Handle<Mesh>,
    &mut Collider,
  )>,
  sensor_query: Query<&MutatorScreen>,
  mut meshes: ResMut<Assets<Mesh>>,
) {
  for event in collision_events.iter() {
    if !state.run {
      continue;
    }

    if let CollisionEvent::Started(collider1, collider2, flags) = event {
      if flags != &CollisionEventFlags::SENSOR {
        continue;
      }

      let is_coll_1_product = product_query.get(*collider1).is_ok();
      let is_coll_2_product = product_query.get(*collider2).is_ok();

      let mutator_screen;
      let (mut product, mut bounce, mut friction, mut mass, mut mesh, mut collider) = if is_coll_1_product {
        mutator_screen = sensor_query.get(*collider2).unwrap();
        product_query.get_mut(*collider1).unwrap()
      } else if is_coll_2_product {
        mutator_screen = sensor_query.get(*collider1).unwrap();
        product_query.get_mut(*collider2).unwrap()
      } else {
        // Exit, nothing to do here
        return;
      };

      match mutator_screen {
        MutatorScreen::Bounce(value) => bounce.coefficient = product.modify_bounce(*value),
        MutatorScreen::Mass(value) => *mass = ColliderMassProperties::Mass(product.modify_mass(*value)),
        MutatorScreen::Friction(value) => friction.coefficient = product.modify_friction(*value),
        MutatorScreen::Shape(new_shape) => {
          let (new_mesh, new_collider) = product.modify_shape(*new_shape, &mut meshes);
          *mesh = new_mesh;
          *collider = new_collider;
        },
      }
    }
  }
}

fn handle_marked_for_deletion(mut commands: Commands, query: Query<Entity, With<MarkForDespawn>>) {
  query.for_each(|e| commands.entity(e).despawn());
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
    app
      .add_startup_system(configure_rapier)
      .add_system(contact_force_events)
      .add_system(collision_events.after(contact_force_events))
      .add_system(handle_marked_for_deletion.in_base_set(PhysicsSet::Writeback));
  }
}
