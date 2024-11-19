use bevy::{ecs::system::EntityCommands, prelude::*};

use super::BUTTON_COLOR;

#[derive(Component)]
pub struct GenerateMazeButton;

#[derive(Component)]
pub struct LoadMazeButton;

#[derive(Component)]
pub struct SaveMazeButton;

pub fn spawn_action_button<'a>(
    builder: &'a mut ChildBuilder,
    component: impl Component,
    text: &str,
) -> EntityCommands<'a> {
    let button_text = |builder: &mut ChildBuilder| {
        builder.spawn(TextBundle {
            text: Text {
                justify: JustifyText::Center,
                sections: vec![TextSection::new(text, TextStyle { ..default() })],
                ..default()
            },
            ..default()
        });
    };

    let mut button = builder.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(180.0),
                height: Val::Px(50.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BUTTON_COLOR.into(),
            ..default()
        },
        component,
    ));

    button.with_children(button_text);

    button
}
