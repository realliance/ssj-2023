use bevy::{app::PluginGroupBuilder, prelude::*};

mod spawner;
pub use spawner::*;

mod product;
pub use product::*;

mod consumer;
pub use consumer::*;

mod collision;
pub use collision::*;

mod camera;
pub use camera::*;

mod state;
pub use state::*;

mod ui;
pub use ui::*;

pub struct GameplayPlugins;

impl PluginGroup for GameplayPlugins {
  fn build(self) -> PluginGroupBuilder {
    PluginGroupBuilder::start::<Self>()
      .add(StatePlugin)
      .add(SpawnerPlugin)
      .add(CollisionPlugin)
      .add(CameraPlugin)
      .add(UiPlugin)
  }
}
