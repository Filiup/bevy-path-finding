use crate::maze::{
    states::MazeState,
    ui::{
        builders::spawn_ui_container,
        buttons::{builders::spawn_action_button, interactions::button_state_system},
    },
};
use bevy::prelude::*;

#[derive(Component)]
pub struct GenerateMazeButton;

#[derive(Component)]
pub struct LoadMazeButton;

#[derive(Component)]
pub struct SaveMazeButton;

pub fn build_main_menu(mut commands: Commands) {
    spawn_ui_container(&mut commands)
        .with_children(|builder| {
            spawn_action_button(builder, GenerateMazeButton, "Generate maze");
        })
        .with_children(|builder| {
            spawn_action_button(builder, SaveMazeButton, "Save maze");
        })
        .with_children(|builder| {
            spawn_action_button(builder, LoadMazeButton, "Load maze");
        });
}

pub(crate) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MazeState::MainMenu), build_main_menu)
            .add_systems(
                Update,
                button_state_system::<GenerateMazeButton, _>(
                    MazeState::Generation,
                    Interaction::Pressed,
                ),
            );
    }
}
