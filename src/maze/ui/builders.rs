use bevy::prelude::*;
use crate::maze::window::{UI_WINDOW_HEIGHT, UI_WINDOW_WIDTH};

pub fn spawn_ui_container<'a>(commands: &'a mut Commands) -> EntityCommands<'a> {
    let container = commands.spawn(Node {
        position_type: PositionType::Absolute,
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(10.0),
        align_self: AlignSelf::Center,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        width: Val::Px(UI_WINDOW_WIDTH),
        height: Val::Px(UI_WINDOW_HEIGHT),
        right: Val::Px(0.0),
        ..default()
    });

    container
}
