use crate::maze::{
    common::states::{MazeState, MenuState},
    grid::ResetGridEvent,
};

use super::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct WithMazeMenu;

pub fn build_main_menu(mut commands: Commands) {
    spawn_ui_container(&mut commands, WithMazeMenu).with_children(|builder| {
        spawn_action_button(builder, SaveMazeButton, "Save maze");
        spawn_action_button(builder, SolveMazeButton, "Solve maze");
        spawn_action_button(builder, ResetMazeButton, "Reset maze");
    });
}

fn reset_maze(
    mut maze_state: ResMut<NextState<MazeState>>,
    mut reset_grid_event_writer: EventWriter<ResetGridEvent>,
    reset_button_interraction_query: Query<&Interaction, With<ResetMazeButton>>,
) {
    let button_clicked = reset_button_interraction_query
        .iter()
        .any(|&i| i == Interaction::Pressed);

    if !button_clicked {
        return;
    }

    reset_grid_event_writer.send(ResetGridEvent);
    maze_state.set(MazeState::MainMenu(MenuState::WithoutMaze));
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
        )
        .add_systems(
            Update,
            (
                button_state_system::<SaveMazeButton, _>(MazeState::MazeSave, Interaction::Pressed),
                button_state_system::<SolveMazeButton, _>(MazeState::MazeSolving, Interaction::Pressed),
            ),
        )
        .add_systems(Update, reset_maze);
    }
}
