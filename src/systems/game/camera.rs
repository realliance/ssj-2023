use bevy::prelude::*;
use smooth_bevy_cameras::controllers::fps::FpsCameraController;

fn handle_camera_pan_right_mouse_btn(buttons: Res<Input<MouseButton>>, mut camera: Query<&mut FpsCameraController>) {
  if let Ok(mut camera) = camera.get_single_mut() {
    let sentitivity = if buttons.pressed(MouseButton::Right) {
      Vec2::splat(0.3)
    } else {
      Vec2::ZERO
    };
    camera.mouse_rotate_sensitivity = sentitivity;
  }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(handle_camera_pan_right_mouse_btn);
  }
}
