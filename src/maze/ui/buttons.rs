use super::{BUTTON_COLOR, BUTTON_HOVER_COLOR};
use bevy::{ecs::system::EntityCommands, prelude::*, state::state::FreelyMutableState};

#[derive(Component)]
pub struct ActionButton;

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

#[allow(clippy::type_complexity)]
pub fn button_state_system<T, S>(
    next_state: S,
    interaction_type: Interaction,
) -> impl Fn(Query<'_, '_, &Interaction, (Changed<Interaction>, With<T>)>, ResMut<'_, NextState<S>>)
where
    T: Component,
    S: States + FreelyMutableState + Copy,
{
    move |button_query: Query<&Interaction, (Changed<Interaction>, With<T>)>,
          mut next_state_res: ResMut<NextState<S>>| {
        for interaction in button_query.into_iter() {
            if *interaction == interaction_type {
                next_state_res.set(next_state);
            }
        }
    }
}

pub fn buttons_hover_changle_color(
    mut buttons_color_query: Query<(&Interaction, &mut BackgroundColor), With<ActionButton>>,
) {
    for (interaction, mut color) in buttons_color_query.iter_mut() {
        match &interaction {
            Interaction::Hovered => *color = BUTTON_HOVER_COLOR.into(),
            Interaction::None => *color = BUTTON_COLOR.into(),
            _ => (),
        }
    }
}

pub struct ActionButtonsPlugin;

impl Plugin for ActionButtonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, buttons_hover_changle_color);
    }
}
