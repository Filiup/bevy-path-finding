#[macro_use]
mod interactions;
mod buttons;
mod slider;

use super::{
    states::MazeState,
    window::{UI_WINDOW_HEIGHT, UI_WINDOW_WIDTH},
};

pub const BUTTON_COLOR: Color = Color::srgb(128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0);
pub const BUTTON_HOVER_COLOR: Color = Color::srgb(173.0 / 255.0, 216.0 / 255.0, 230.0 / 255.0);

use bevy::color::Color;
use bevy::prelude::*;
use buttons::{spawn_action_button, GenerateMazeButton, LoadMazeButton, SaveMazeButton};
use interactions::{button_hover_change_color, button_state_system};
use slider::{change_slider_state, change_slider_value, move_slider, spawn_slider};

pub fn generate_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
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
            },
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
            spawn_slider(builder);
        });
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_ui).add_systems(
            Update,
            (
                button_state_system::<GenerateMazeButton, _>(
                    MazeState::Generation,
                    Interaction::Pressed,
                ),
                create_buttons_hover_system!(GenerateMazeButton, SaveMazeButton, LoadMazeButton),
                change_slider_state,
                move_slider,
                change_slider_value,
            ),
        );
    }
}
