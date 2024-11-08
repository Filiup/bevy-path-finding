use bevy::{prelude::*, window::PrimaryWindow};

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
