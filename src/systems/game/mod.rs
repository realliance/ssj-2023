use bevy::{app::PluginGroupBuilder, prelude::*};

mod spawner;
pub use spawner::*;

mod product;
pub use product::*;

mod consumer;
pub use consumer::*;

mod collision;
pub use collision::*;

pub struct GameplayPlugins;

impl PluginGroup for GameplayPlugins {
  fn build(self) -> PluginGroupBuilder {
    PluginGroupBuilder::start::<Self>()
      .add(SpawnerPlugin)
      .add(CollisionPlugin)
  }
}
