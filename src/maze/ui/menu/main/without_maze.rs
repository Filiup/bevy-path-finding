use super::*;
use crate::maze::common::states::{MazeState, MenuState};
use bevy::prelude::*;

#[derive(Component)]
pub struct WithoutMazeMenu;

pub fn build_main_menu(mut commands: Commands) {
    spawn_ui_container(&mut commands, WithoutMazeMenu)
        .with_children(|builder| {
            spawn_action_button(builder, GenerateMazeButton, "Generate maze");
        })
        .with_children(|builder| {
            spawn_action_button(builder, LoadMazeButton, "Load maze");
        });
}

pub(crate) struct WithoutMazeMenuPlugin;

impl Plugin for WithoutMazeMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MazeState::MainMenu(MenuState::WithoutMaze)),
            build_main_menu,
        )
        .add_systems(
            OnExit(MazeState::MainMenu(MenuState::WithoutMaze)),
            despawn_menu::<WithoutMazeMenu>,
        )
        .add_systems(
            Update,
            (
                button_state_system::<GenerateMazeButton, _>(
                    MazeState::MazeGeneration,
                    Interaction::Pressed,
                ),
                button_state_system::<LoadMazeButton, _>(MazeState::MazeLoad, Interaction::Pressed),
            ),
        );
    }
}
