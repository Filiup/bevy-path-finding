use crate::maze::{
    common::{
        cell::StartCell,
        states::{MazeState, SolveState},
    },
    constants::grid::START_CELL_COLOR,
};

use super::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct StartSelectMenu;

pub struct StartSelectMenuPlugin;

pub fn build_menu(mut commands: Commands) {
    spawn_ui_container(&mut commands, StartSelectMenu).with_children(|builder| {
        builder.spawn((
            Text::new("Select maze start"),
            TextLayout::new_with_justify(JustifyText::Center),
        ));
    });
}

impl Plugin for StartSelectMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MazeState::MazeSolve(SolveState::StartSelect)),
            build_menu,
        )
        .add_systems(
            OnExit(MazeState::MazeSolve(SolveState::StartSelect)),
            despawn_menu::<StartSelectMenu>,
        )
        .add_systems(
            Update,
            (
                hover_cells_system(START_CELL_COLOR),
                set_cell_path_system(
                    StartCell,
                    MazeState::MazeSolve(SolveState::EndSelect),
                    START_CELL_COLOR,
                ),
            )
                .run_if(in_state(MazeState::MazeSolve(SolveState::StartSelect))),
        );
    }
}
