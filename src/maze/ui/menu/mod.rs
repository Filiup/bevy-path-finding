pub mod generate;
pub mod main;

use bevy::prelude::*;
use generate::GenerateMenuPlugin;
use main::MainMenuPlugin;

use crate::maze::window::{UI_WINDOW_HEIGHT, UI_WINDOW_WIDTH};

pub struct MenuPlugin;

pub fn spawn_ui_container<'a>(
    commands: &'a mut Commands,
    menu_marker_component: impl Component,
) -> EntityCommands<'a> {
    let container = commands.spawn((
        Node {
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
        },
        menu_marker_component,
    ));

    container
}

pub fn despawn_menu<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(MainMenuPlugin)
            .add_plugins(GenerateMenuPlugin);
    }
}
