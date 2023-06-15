use bevy::app::App;
use bevy_rapier3d::prelude::*;

use crate::systems::{setup_graphics, BevyPlugins, GameplayPlugins};

pub fn run() {
  App::default()
    .add_plugins(BevyPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugins(GameplayPlugins)
    .add_startup_system(setup_graphics)
    .run();
}
