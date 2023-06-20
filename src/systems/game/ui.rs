use bevy::prelude::*;
use bevy_egui::{
  egui::{self, Align2, Layout},
  EguiContexts,
};
use serde::Deserialize;

use super::{get_shop_items, ClearProducts, GameState, ShopItem};

fn ui_system(
  mut contexts: EguiContexts,
  mut state: ResMut<GameState>,
  mut clear_products: EventWriter<ClearProducts>,
  shop_items: Res<ShopItems>,
  opening_text: Res<OpeningText>,
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  camera: Query<&Transform, With<Camera>>,
) {
  if state.start_popup {
    egui::Window::new(&opening_text.title)
      .anchor(Align2::CENTER_CENTER, [0.0, 0.0])
      .scroll2([false, true])
      .show(contexts.ctx_mut(), |ui| {
        ui.wrap_text();
        ui.label(&opening_text.body);

        if ui.button("Begin").clicked() {
          state.run = true;
          state.show_shop = true;
          state.start_popup = false;
        }
      });
  }

  if !state.show_shop {
    return;
  }

  egui::Window::new("Operations Menu")
    .scroll2([false, true])
    .show(contexts.ctx_mut(), |ui| {
      if state.won {
        ui.label("You Win!");
        state.run = false;
        return;
      }

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

      let button_text: &str = if state.run { "Hives Running" } else { "Purchase" };
      shop_items.0.iter().for_each(|item| {
        ui.with_layout(Layout::top_down(egui::Align::Max), |ui| {
          ui.vertical(|ui| {
            ui.label(item.name);
            ui.label(format!("Cost: {}", item.cost));
          });

          let can_purchase = state.currency >= item.cost;

          if ui
            .add_enabled(can_purchase && !state.run, egui::Button::new(button_text))
            .clicked()
          {
            if let Some(trans) = camera.iter().next() {
              state.currency -= item.cost;

              let position = trans.forward() * 10.0;
              (item.create_purchase)(&mut state, &mut commands, &mut meshes, &mut materials, position);
            } else {
              warn!("Camera missing?");
            }
          }
        });
      });
    });
}

pub const OPENING_TEXT: &'static str = include_str!("../../../static/opening_text.toml");

#[derive(Deserialize, Resource)]
pub struct OpeningText {
  pub title: String,
  pub body: String,
}

impl Default for OpeningText {
  fn default() -> Self {
    toml::from_str(OPENING_TEXT).unwrap()
  }
}

#[derive(Resource)]
pub struct ShopItems(pub Vec<ShopItem>);

pub struct UiPlugin;

impl Plugin for UiPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(ShopItems(get_shop_items()))
      .init_resource::<OpeningText>()
      .add_system(ui_system);
  }
}
