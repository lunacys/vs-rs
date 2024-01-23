use bevy::prelude::*;

mod camera;
mod player;
mod tilemap;

#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraMovementPlugin;
use player::PlayerPlugin;
use tilemap::TileMapPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "VS-RS".into(),
            resolution: (1600., 900.).into(),
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(TileMapPlugin)
    .add_plugins(CameraMovementPlugin)
    .add_plugins(PlayerPlugin)
    .add_systems(Startup, spawn_camera);

    #[cfg(debug_assertions)]
    app.add_plugins(WorldInspectorPlugin::new());

    app.add_systems(Update, bevy::window::close_on_esc);

    app.run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
