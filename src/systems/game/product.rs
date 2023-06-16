//! Product Logic

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use derive_builder::Builder;

#[derive(Clone, Copy, Debug)]
pub enum ProductShape {
  Sphere,
  Cube,
  Cylinder,
  Disc,
}

impl ProductShape {
  pub fn base_value(&self) -> f32 {
    match self {
      ProductShape::Sphere => 1.0,
      ProductShape::Cube => 1.5,
      ProductShape::Cylinder => 2.0,
      ProductShape::Disc => 2.5,
    }
  }

  pub fn collider(&self, size: f32) -> Collider {
    match self {
      ProductShape::Sphere => Collider::ball(size),
      ProductShape::Cube => todo!(),
      ProductShape::Cylinder => todo!(),
      ProductShape::Disc => todo!(),
    }
  }

  pub fn pbr_bundle(
    &self,
    position: Vec3,
    size: f32,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
  ) -> PbrBundle {
    let material = materials.add(Color::SILVER.into());
    let transform = Transform::from_translation(position);

    match self {
      ProductShape::Sphere => PbrBundle {
        mesh: meshes.add(
          shape::UVSphere {
            radius: size,
            ..Default::default()
          }
          .into(),
        ),
        material,
        transform,
        ..Default::default()
      },
      ProductShape::Cube => todo!(),
      ProductShape::Cylinder => todo!(),
      ProductShape::Disc => todo!(),
    }
  }
}

impl Default for ProductShape {
  fn default() -> Self {
    Self::Sphere
  }
}

/// Represents a product, produced by a spawner and consumed by a consumer.
#[derive(Component, Builder, Clone, Debug)]
#[builder(default)]
pub struct Product {
  shape: ProductShape,
  hit_mult: f32,
  hit_count: u32,
  mass: f32,
  bounce: f32,
  friction: f32,
  size: f32,
}

impl Default for Product {
  fn default() -> Self {
    Self {
      shape: Default::default(),
      mass: 1.0,
      hit_mult: 1.0,
      hit_count: 0,
      bounce: 1.0,
      friction: 1.0,
      size: 1.0,
    }
  }
}

impl Product {
  pub fn payment(&self) -> u32 {
    (self.shape.base_value()
      * self.hit_mult
      * (1.0 + (self.bounce - 1.0).abs())
      * (1.0 + (self.friction - 1.0).abs())
      * (1.0 + (self.mass - 1.0).abs()))
    .floor() as u32
  }

  pub fn add_hit_mult(&mut self, force_mag: f32) {
    self.hit_mult += force_mag / 10000.0;
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
      .insert(self.shape.collider(self.size))
      .insert(TransformBundle::from(Transform::from_translation(
        position.clone().into(),
      )))
      .insert(RigidBody::Dynamic)
      .insert(Restitution::coefficient(self.bounce))
      .insert(Friction {
        coefficient: self.friction,
        ..Default::default()
      })
      .insert(ActiveEvents::CONTACT_FORCE_EVENTS)
      .insert(self.shape.pbr_bundle(position.into(), self.size, meshes, materials))
      .id()
  }
}
