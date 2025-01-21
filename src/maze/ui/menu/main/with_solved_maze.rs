use crate::maze::{
    common::{
        cell::MazeCell,
        states::{MazeState, MenuState},
    },
    constants::grid::CELL_COLOR,
};

use super::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct WithSolvedMazeMenu;

pub struct WithSolvedMazeMenuPlugin;

#[derive(Component)]
pub struct ResetButton;

pub fn reset_cell_colors(
    mut cell_sprite_query: Query<&mut Sprite, With<MazeCell>>,
    reset_button_interraction_query: Query<&Interaction, With<ResetButton>>,
) {
    let button_clicked = reset_button_interraction_query
        .iter()
        .any(|&i| i == Interaction::Pressed);

    if !button_clicked {
        return;
    }

    for mut sprite in cell_sprite_query.iter_mut() {
        sprite.color = CELL_COLOR;
    }
}

pub fn build_main_menu(mut commands: Commands) {
    spawn_ui_container(&mut commands, WithSolvedMazeMenu).with_children(|builder| {
        spawn_action_button(builder, ResetButton, "Reset");
    });
}

impl Plugin for WithSolvedMazeMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MazeState::MainMenu(MenuState::WithSolvedMaze)),
            build_main_menu,
        )
        .add_systems(
            OnExit(MazeState::MainMenu(MenuState::WithSolvedMaze)),
            despawn_menu::<WithSolvedMazeMenu>,
        )
        .add_systems(
            Update,
            (
                button_state_system::<ResetButton, _>(
                    MazeState::MainMenu(MenuState::WithMaze),
                    Interaction::Pressed,
                ),
                reset_cell_colors,
            ),
        );
    }
}
