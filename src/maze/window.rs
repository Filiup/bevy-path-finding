use bevy::prelude::*;
use bevy::{window::PrimaryWindow, window::WindowResolution};

pub const WINDOW_HEIGHT: f32 = 600.0;

pub const GRID_WINDOW_WIDTH: f32 = 800.0;
pub const GRID_WINDOW_HEIGHT: f32 = WINDOW_HEIGHT;

pub const UI_WINDOW_WIDTH: f32 = 200.0;
pub const UI_WINDOW_HEIGHT: f32 = WINDOW_HEIGHT;

pub const WINDOW_WIDTH: f32 = GRID_WINDOW_WIDTH + UI_WINDOW_WIDTH;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let primary_window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            primary_window.width() / 2.0,
            primary_window.height() / 2.0,
            0.0,
        ),
        ..default()
    });
}

pub struct MazeWindowPlugin;

impl Plugin for MazeWindowPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, spawn_camera);
    }
}
