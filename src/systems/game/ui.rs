use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use super::{ClearProducts, GameState};

fn ui_system(mut contexts: EguiContexts, mut state: ResMut<GameState>, mut clear_products: EventWriter<ClearProducts>) {
  egui::Window::new("Operations Menu").show(contexts.ctx_mut(), |ui| {
    ui.vertical_centered_justified(|ui| {
      ui.label(format!("Bug Guts: {}", state.currency));
    });
    let run_button_text = if state.run { "Stop Hives" } else { "Start Hives" };
    ui.vertical_centered_justified(|ui| {
      if ui.button(run_button_text).clicked() {
        clear_products.send_default();
        state.toggle_run();
      }
    });
  });
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(ui_system);
  }
}
