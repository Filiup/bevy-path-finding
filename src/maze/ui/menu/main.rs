use crate::maze::{
    common::states::{MazeState, MenuState},
    ui::buttons::{builders::spawn_action_button, interactions::button_state_system},
};
use bevy::prelude::*;

use super::*;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct GenerateMazeButton;

#[derive(Component)]
pub struct LoadMazeButton;

pub fn build_main_menu(mut commands: Commands) {
    spawn_ui_container(&mut commands, MainMenu)
        .with_children(|builder| {
            spawn_action_button(builder, GenerateMazeButton, "Generate maze");
        })
        .with_children(|builder| {
            spawn_action_button(builder, LoadMazeButton, "Load maze");
        });
}

pub(crate) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MazeState::MainMenu(MenuState::WithoutMaze)),
            build_main_menu,
        )
        .add_systems(
            OnExit(MazeState::MainMenu(MenuState::WithoutMaze)),
            despawn_menu::<MainMenu>,
        )
        .add_systems(
            Update,
            button_state_system::<GenerateMazeButton, _>(
                MazeState::MazeGeneration,
                Interaction::Pressed,
            ),
        );
    }
}
