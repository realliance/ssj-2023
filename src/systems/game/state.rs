use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
  /// The players current amount of money
  pub currency: u32,

  /// Run Spawners?
  pub run: bool,

  /// Show shop?
  pub show_shop: bool,

  /// Won Game?
  pub won: bool,

  // Show Start Game Pop Up?
  pub start_popup: bool,
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
      show_shop: false,
      run: false,
      won: false,
      start_popup: true,
    }
  }
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    app.init_resource::<GameState>();
  }
}
