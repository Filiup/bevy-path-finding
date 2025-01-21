use crate::maze::{
    common::states::{MazeState, SolveState},
    constants::grid::END_CELL_COLOR,
};

use super::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct EndSelectMenu;

pub struct EndSelectMenuPlugin;

pub fn build_menu(mut commands: Commands) {
    spawn_ui_container(&mut commands, EndSelectMenu).with_children(|builder| {
        builder.spawn((
            Text::new("Select maze end"),
            TextLayout::new_with_justify(JustifyText::Center),
        ));
    });
}

impl Plugin for EndSelectMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MazeState::MazeSolve(SolveState::EndSelect)),
            build_menu,
        )
        .add_systems(
            OnExit(MazeState::MazeSolve(SolveState::EndSelect)),
            despawn_menu::<EndSelectMenu>,
        )
        .add_systems(
            Update,
            (
                hover_cells_system(END_CELL_COLOR),
                set_cell_path_system(
                    EndCell,
                    MazeState::MazeSolve(SolveState::Solving),
                    END_CELL_COLOR,
                ),
            )
                .run_if(in_state(MazeState::MazeSolve(SolveState::EndSelect))),
        );
    }
}
