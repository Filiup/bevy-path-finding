use super::*;
use bevy::prelude::*;
use std::time::Duration;

use crate::maze::{
    common::{
        solving::MazeSolvingTimer,
        states::{MazeState, SolveState},
    },
    constants::iterration::DEFAULT_MAZE_SOLVING_TIMER_VALUE,
    ui::sliders::{builders::spawn_slider, SliderDirection, SliderHandle},
};

#[derive(Component)]
pub struct SolvingMenu;

#[derive(Component)]
pub struct SolvingSpeedSlider;

pub(crate) struct SolvingMenuPlugin;

pub fn change_solving_timer(
    mut solving_timer: ResMut<MazeSolvingTimer>,
    slider_handle_query: Query<&SliderHandle, (With<SolvingSpeedSlider>, Changed<SliderHandle>)>,
) {
    let slider_handle = slider_handle_query.get_single();

    if let Ok(slider_handle) = slider_handle {
        let new_duration = Duration::from_millis(slider_handle.current_value);
        if solving_timer.timer.duration() != new_duration {
            solving_timer.timer.set_duration(new_duration);
            solving_timer.timer.reset();
        }
    }
}

pub fn build_solving_menu(mut commands: Commands) {
    spawn_ui_container(&mut commands, SolvingMenu)
        .with_children(|builder| {
            builder.spawn(Text::new("Solving speed"));
        })
        .with_children(|builder| {
            spawn_slider(
                builder,
                DEFAULT_MAZE_SOLVING_TIMER_VALUE,
                SliderDirection::Descending,
                SolvingSpeedSlider,
            );
        });
}

impl Plugin for SolvingMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MazeState::MazeSolve(SolveState::Solving)),
            build_solving_menu,
        )
        .add_systems(
            OnExit(MazeState::MazeSolve(SolveState::Solving)),
            despawn_menu::<SolvingMenu>,
        )
        .add_systems(
            Update,
            change_solving_timer.run_if(in_state(MazeState::MazeSolve(SolveState::Solving))),
        );
    }
}
