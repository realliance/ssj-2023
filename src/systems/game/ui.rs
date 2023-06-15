use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

fn ui_system(mut contexts: EguiContexts) {
  egui::Window::new("Sim Menu").show(
    contexts.ctx_mut(),
    |ui| {
      if ui.button("This doesn't work yet").clicked() {}
    },
  );
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(ui_system);
  }
}
