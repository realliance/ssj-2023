use bevy::app::App;

use crate::systems::{setup_graphics, BevyPlugins};

pub fn run() {
  App::default()
    .add_plugins(BevyPlugins)
    .add_startup_system(setup_graphics)
    .run();
}
