mod buttons;
mod sliders;

use super::{
    states::MazeState,
    window::{UI_WINDOW_HEIGHT, UI_WINDOW_WIDTH},
};

use bevy::prelude::*;
use buttons::{
    builders::spawn_action_button, interactions::button_state_system, ActionButtonsPlugin,
};
use sliders::{builders::spawn_slider, SliderDirection, SlidersPlugin};

#[derive(Component)]
pub struct ChangeGenerationSpeedSlider;

#[derive(Component)]
pub struct GenerateMazeButton;

#[derive(Component)]
pub struct LoadMazeButton;

#[derive(Component)]
pub struct SaveMazeButton;

pub fn generate_ui(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            align_self: AlignSelf::Center,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Px(UI_WINDOW_WIDTH), //
            height: Val::Px(UI_WINDOW_HEIGHT),
            right: Val::Px(0.0),
            ..default()
        })
        .with_children(|builder| {
            spawn_action_button(builder, GenerateMazeButton, "Generate maze");
        })
        .with_children(|builder| {
            spawn_action_button(builder, SaveMazeButton, "Save maze");
        })
        .with_children(|builder| {
            spawn_action_button(builder, LoadMazeButton, "Load maze");
        })
        .with_children(|builder| {
            spawn_slider(
                builder,
                1000,
                SliderDirection::Descending,
                ChangeGenerationSpeedSlider,
            );
        });
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SlidersPlugin)
            .add_plugins(ActionButtonsPlugin)
            .add_systems(Startup, generate_ui)
            .add_systems(
                Update,
                button_state_system::<GenerateMazeButton, _>(
                    MazeState::Generation,
                    Interaction::Pressed,
                ),
            );
    }
}
