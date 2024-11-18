mod buttons;

use bevy:: prelude::*;
use buttons::spawn_action_button;
use super::window::{UI_WINDOW_HEIGHT, UI_WINDOW_WIDTH};


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
            spawn_action_button(builder, "Generate maze");
        })
        .with_children(|builder| {
            spawn_action_button(builder, "Save maze");
        })
        .with_children(|builder| {
            spawn_action_button(builder, "Load maze");
        });
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_ui);
    }
}
