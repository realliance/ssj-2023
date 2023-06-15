use bevy::{app::App, prelude::Msaa};
use bevy_egui::EguiPlugin;
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::{controllers::fps::FpsCameraPlugin, LookTransformPlugin};

use crate::systems::{setup_graphics, BevyPlugins, GameplayPlugins};

pub fn run() {
  App::default()
    .insert_resource(Msaa::Sample4)
    .add_plugins(BevyPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_plugin(EguiPlugin)
    .add_plugins(GameplayPlugins)
    .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
    .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::default())
    .add_plugin(LookTransformPlugin)
    .add_plugin(FpsCameraPlugin::default())
    .add_startup_system(setup_graphics)
    .run();
}
