use bevy::prelude::*;

pub fn spawn_button<'a>(
    builder: &'a mut ChildBuilder,
    kind_component: impl Component,
    id_component: impl Component,
    width: Val,
    height: Val,
    color: Color,
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
        BackgroundColor(color),
        Node {
            width,
            height,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Button,
        id_component,
        kind_component,
    ));

    button.with_children(button_text);

    button
}
