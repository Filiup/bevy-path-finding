pub mod generate;
pub mod load;
pub mod main;
pub mod save;

use bevy::prelude::*;
use generate::GenerateMenuPlugin;
use load::LoadMenuPlugin;
use main::MainMenuPlugin;
use save::SaveMenuPlugin;

use crate::maze::constants::{
    ui::{
        ACTION_BUTTON_COLOR, ACTION_BUTTON_HEIGHT, ACTION_BUTTON_HOVER_COLOR, ACTION_BUTTON_WIDTH,
    },
    window::{UI_WINDOW_HEIGHT, UI_WINDOW_WIDTH},
};

use super::buttons::builders::spawn_button;

#[derive(Component)]
pub struct ActionButton;

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

fn spawn_slot_container<'a>(builder: &'a mut ChildBuilder) -> EntityCommands<'a> {
    let container = builder.spawn(Node {
        display: Display::Grid,
        grid_template_columns: RepeatedGridTrack::flex(5, 1.0),
        grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
        row_gap: Val::Px(10.0),
        column_gap: Val::Px(10.0),
        ..default()
    });

    container
}

pub fn despawn_menu<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn spawn_action_button<'a>(
    builder: &'a mut ChildBuilder,
    component: impl Component,
    text: &str,
) -> EntityCommands<'a> {
    spawn_button(
        builder,
        ActionButton,
        component,
        ACTION_BUTTON_WIDTH,
        ACTION_BUTTON_HEIGHT,
        ACTION_BUTTON_COLOR,
        text,
    )
}

pub(crate) fn change_action_button_color(
    mut buttons_color_query: Query<(&Interaction, &mut BackgroundColor), With<ActionButton>>,
) {
    for (interaction, mut color) in buttons_color_query.iter_mut() {
        match &interaction {
            Interaction::Hovered => *color = ACTION_BUTTON_HOVER_COLOR.into(),
            Interaction::None => *color = ACTION_BUTTON_COLOR.into(),
            _ => (),
        }
    }
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, change_action_button_color)
            .add_plugins(MainMenuPlugin)
            .add_plugins(GenerateMenuPlugin)
            .add_plugins(SaveMenuPlugin)
            .add_plugins(LoadMenuPlugin);
    }
}
