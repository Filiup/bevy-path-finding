use super::constants::window::*;
use bevy::prelude::*;
use bevy::{window::PrimaryWindow, window::WindowResolution};

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let primary_window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2d,
        Transform::from_xyz(
            primary_window.width() / 2.0,
            primary_window.height() / 2.0,
            0.0,
        ),
    ));
}

pub struct MazeDefaultPlugin;

impl Plugin for MazeDefaultPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                title: "Path finding visualization".to_string(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, spawn_camera);
    }
}
