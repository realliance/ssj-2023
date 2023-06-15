use bevy::{app::PluginGroupBuilder, prelude::*};

mod spawner;
pub use spawner::*;

mod product;
pub use product::*;

pub struct GameplayPlugins;

impl PluginGroup for GameplayPlugins {
  fn build(self) -> PluginGroupBuilder {
    PluginGroupBuilder::start::<Self>().add(SpawnerPlugin)
  }
}
