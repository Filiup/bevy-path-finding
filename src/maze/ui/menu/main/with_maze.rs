use crate::maze::common::states::{MazeState, MenuState};

use super::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct WithMazeMenu;

pub fn build_main_menu(mut commands: Commands) {
    spawn_ui_container(&mut commands, WithMazeMenu).with_children(|builder| {
        spawn_action_button(builder, SaveMazeButton, "Save maze");
    });
}

pub struct WithMazeMenuPlugin;

impl Plugin for WithMazeMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MazeState::MainMenu(MenuState::WithMaze)),
            build_main_menu,
        )
        .add_systems(
            OnExit(MazeState::MainMenu(MenuState::WithMaze)),
            despawn_menu::<WithMazeMenu>,
        );
    }
}
