use bevy::{ecs::system::EntityCommands, prelude::*};

pub fn spawn_action_button<'a>(builder: &'a mut ChildBuilder, text: &str) -> EntityCommands<'a> {
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

    let mut button = builder.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(180.0),
            height: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: Color::srgb(255.0, 0.0, 0.0).into(),
        ..default()
    });

    button.with_children(button_text);

    button
}
