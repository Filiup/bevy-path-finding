use bevy::{
    prelude::{Changed, NextState, Query, ResMut, With},
    ui::Interaction,
};

use super::buttons::GenerateMazeButton;
use crate::maze::states::MazeState;

pub fn button_generate_clicked(
    button_query: Query<&Interaction, (Changed<Interaction>, With<GenerateMazeButton>)>,
    mut maze_next_state: ResMut<NextState<MazeState>>,
) {
    for interaction in button_query.iter() {
        if let Interaction::Pressed = interaction {
            maze_next_state.set(MazeState::Generation);
        }
    }
}
