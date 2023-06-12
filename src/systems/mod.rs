use bevy::{
  a11y::AccessibilityPlugin, app::PluginGroupBuilder, core_pipeline::CorePipelinePlugin, input::InputPlugin,
  log::LogPlugin, pbr::PbrPlugin, prelude::*, render::RenderPlugin, scene::ScenePlugin, sprite::SpritePlugin,
  text::TextPlugin, time::TimePlugin, ui::UiPlugin, winit::WinitPlugin,
};

pub fn setup_graphics(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands.spawn(Camera3dBundle {
    transform: Transform::from_xyz(-30.0, 30.0, 100.0).looking_at(Vec3::new(0.0, 10.0, 0.0), Vec3::Y),
    ..Default::default()
  });

  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 9000.0,
      range: 100.,
      shadows_enabled: true,
      ..Default::default()
    },
    transform: Transform::from_xyz(8.0, 16.0, 8.0),
    ..Default::default()
  });

  commands.spawn(PbrBundle {
    mesh: meshes.add(shape::Plane::from_size(50.0).into()),
    material: materials.add(Color::SILVER.into()),
    transform: Transform::from_xyz(0.0, -10.0, 0.0),
    ..Default::default()
  });
}

pub struct BevyPlugins;

impl PluginGroup for BevyPlugins {
  fn build(self) -> bevy::app::PluginGroupBuilder {
    PluginGroupBuilder::start::<Self>()
      // Bevy Plugins
      .add(LogPlugin::default())
      .add(TaskPoolPlugin::default())
      .add(TypeRegistrationPlugin::default())
      .add(FrameCountPlugin::default())
      .add(AssetPlugin::default())
      .add(TimePlugin::default())
      .add(TransformPlugin::default())
      .add(InputPlugin::default())
      .add(ScenePlugin::default())
      .add(WindowPlugin {
      primary_window: Some(Window {
          fit_canvas_to_parent: true,
          canvas: Some("#bevy".to_string()),
          ..Default::default()
        }),
        ..Default::default()
      })
      .add(AccessibilityPlugin)
      .add(WinitPlugin::default())
      .add(RenderPlugin::default())
      .add(ImagePlugin::default())
      .add(CorePipelinePlugin::default())
      .add(SpritePlugin::default())
      .add(TextPlugin::default())
      .add(UiPlugin::default())
      .add(PbrPlugin::default())
  }
}
