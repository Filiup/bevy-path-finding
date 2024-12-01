use super::*;
use bevy::prelude::*;

pub fn spawn_action_button<'a>(
    builder: &'a mut ChildBuilder,
    component: impl Component,
    text: &str,
) -> EntityCommands<'a> {
    let button_text = |builder: &mut ChildBuilder| {
        builder.spawn((
            Node {
                justify_content: JustifyContent::Center,
                ..default()
            },
            Text::new(text),
        ));
    };

    let mut button = builder.spawn((
        BackgroundColor(BUTTON_COLOR),
        Node {
            width: Val::Px(180.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Button,
        ActionButton,
        component,
    ));

    button.with_children(button_text);

    button
}
