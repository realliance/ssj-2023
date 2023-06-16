use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
  /// The players current amount of money
  pub currency: u32,

  /// Run Spawners?
  pub run: bool,
}

impl GameState {
  pub fn toggle_run(&mut self) {
    self.run = !self.run;
  }
}

impl Default for GameState {
  fn default() -> Self {
    Self {
      currency: Default::default(),
      run: true,
    }
  }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.init_resource::<GameState>();
  }
}
