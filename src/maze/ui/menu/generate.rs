use bevy::prelude::*;

use crate::maze::{
    states::MazeState,
    ui::sliders::{builders::spawn_slider, SliderDirection},
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
                1000,
                SliderDirection::Descending,
                GenerationSpeedSlider,
            );
        });
}

impl Plugin for GenerateMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MazeState::Generation), build_generate_menu)
            .add_systems(OnExit(MazeState::Generation), despawn_menu::<GenerateMenu>);
    }
}
