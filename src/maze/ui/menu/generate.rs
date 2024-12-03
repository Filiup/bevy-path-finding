use std::time::Duration;

use bevy::prelude::*;

use crate::maze::{
    common::generation::MazeGenerationTimer,
    constants::generation::DEFAULT_MAZE_GENERATION_TIMER_VALUE,
    states::MazeState,
    ui::sliders::{builders::spawn_slider, SliderDirection, SliderHandle},
};

use super::*;

#[derive(Component)]
pub struct GenerateMenu;

#[derive(Component)]
pub struct GenerationSpeedSlider;

pub(crate) struct GenerateMenuPlugin;

pub fn build_generate_menu(mut commands: Commands) {
    spawn_ui_container(&mut commands, GenerateMenu)
        .with_children(|builder| {
            builder.spawn(Text::new("Generation speed"));
        })
        .with_children(|builder| {
            spawn_slider(
                builder,
                DEFAULT_MAZE_GENERATION_TIMER_VALUE,
                SliderDirection::Descending,
                GenerationSpeedSlider,
            );
        });
}

pub fn change_generation_timer(
    mut cell_timer: ResMut<MazeGenerationTimer>,
    slider_handle_query: Query<&SliderHandle, (With<GenerationSpeedSlider>, Changed<SliderHandle>)>,
) {
    let slider_handle = slider_handle_query.get_single();

    if let Ok(slider_handle) = slider_handle {
        let new_duration = Duration::from_millis(slider_handle.current_value);
        if cell_timer.timer.duration() != new_duration {
            cell_timer.timer.set_duration(new_duration);
            cell_timer.timer.reset();
        }
    }
}

impl Plugin for GenerateMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MazeState::Generation), build_generate_menu)
            .add_systems(OnExit(MazeState::Generation), despawn_menu::<GenerateMenu>)
            .add_systems(
                Update,
                change_generation_timer.run_if(in_state(MazeState::Generation)),
            );
    }
}
