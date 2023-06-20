use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{GameState, MutatorScreen, ProductShape, SpawnerBuilder};

pub struct ShopItem {
  pub name: &'static str,
  pub cost: u32,
  pub create_purchase: fn(
    state: &mut GameState,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
  ),
}

pub fn get_shop_items() -> Vec<ShopItem> {
  return vec![
    ShopItem {
      name: "Supply Ship with Food",
      cost: 100000,
      create_purchase: |state, _commands, _meshes, _materials, _position| {
        state.won = true;
      },
    },
    ShopItem {
      name: "Panel",
      cost: 10,
      create_purchase: |_state, commands, meshes, materials, position| {
        commands
          .spawn(PbrBundle {
            mesh: meshes.add(shape::Box::new(8.0, 1.0, 8.0).into()),
            material: materials.add(Color::BLACK.into()),
            transform: Transform::from_translation(position),
            ..Default::default()
          })
          .insert(RigidBody::Fixed)
          .insert(Collider::cuboid(4.0, 0.5, 4.0))
          .insert(bevy_mod_picking::PickableBundle::default())
          .insert(bevy_transform_gizmo::GizmoTransformable);
      },
    },
    ShopItem {
      name: "Hive",
      cost: 500,
      create_purchase: |_state, commands, meshes, materials, position| {
        SpawnerBuilder::default()
          .build()
          .unwrap()
          .into_entity(commands, meshes, materials, position);
      },
    },
    ShopItem {
      name: "Cubifier",
      cost: 100,
      create_purchase: |_state, commands, meshes, materials, position| {
        MutatorScreen::Shape(ProductShape::Cube).into_entity(commands, meshes, materials, position);
      },
    },
    ShopItem {
      name: "Cylindifier",
      cost: 100,
      create_purchase: |_state, commands, meshes, materials, position| {
        MutatorScreen::Shape(ProductShape::Cylinder).into_entity(commands, meshes, materials, position);
      },
    },
    ShopItem {
      name: "Discifier",
      cost: 100,
      create_purchase: |_state, commands, meshes, materials, position| {
        MutatorScreen::Shape(ProductShape::Disc).into_entity(commands, meshes, materials, position);
      },
    },
    ShopItem {
      name: "Sphere-Reverter",
      cost: 100,
      create_purchase: |_state, commands, meshes, materials, position| {
        MutatorScreen::Shape(ProductShape::Sphere).into_entity(commands, meshes, materials, position);
      },
    },
    ShopItem {
      name: "Mass-Increaser",
      cost: 100,
      create_purchase: |_state, commands, meshes, materials, position| {
        MutatorScreen::Mass(0.5).into_entity(commands, meshes, materials, position);
      },
    },
    ShopItem {
      name: "Mass-Decreaser",
      cost: 100,
      create_purchase: |_state, commands, meshes, materials, position| {
        MutatorScreen::Mass(-0.5).into_entity(commands, meshes, materials, position);
      },
    },
    ShopItem {
      name: "Bounce-Increaser",
      cost: 100,
      create_purchase: |_state, commands, meshes, materials, position| {
        MutatorScreen::Bounce(0.1).into_entity(commands, meshes, materials, position);
      },
    },
    ShopItem {
      name: "Bounce-Decreaser",
      cost: 100,
      create_purchase: |_state, commands, meshes, materials, position| {
        MutatorScreen::Bounce(-0.1).into_entity(commands, meshes, materials, position);
      },
    },
    ShopItem {
      name: "Friction-Increaser",
      cost: 100,
      create_purchase: |_state, commands, meshes, materials, position| {
        MutatorScreen::Friction(0.2).into_entity(commands, meshes, materials, position);
      },
    },
    ShopItem {
      name: "Friction-Decreaser",
      cost: 100,
      create_purchase: |_state, commands, meshes, materials, position| {
        MutatorScreen::Friction(-0.2).into_entity(commands, meshes, materials, position);
      },
    },
  ];
}
