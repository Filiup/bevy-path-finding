use bevy::{
    prelude::{Changed, Component, Mut, NextState, Query, ResMut, States, With},
    state::state::FreelyMutableState,
    ui::{BackgroundColor, Interaction},
};

use super::{BUTTON_COLOR, BUTTON_HOVER_COLOR};

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

pub fn button_hover_change_color<'a>(
    button_color_query: impl Iterator<Item = (&'a Interaction, Mut<'a, BackgroundColor>)>,
) {
    for (interaction, mut color) in button_color_query {
        match &interaction {
            Interaction::Hovered => *color = BUTTON_HOVER_COLOR.into(),
            Interaction::None => *color = BUTTON_COLOR.into(),
            _ => (),
        }
    }
}

macro_rules! create_filters {
    ($struct_name:ident) => {
        With<$struct_name>
    };
    ($($struct_name:ident),+ $(,)?) => {
       Or<($(With<$struct_name>,)*)>
    };
}

macro_rules! create_buttons_hover_system {
    ($($struct_name:ident),*) => {
        move |mut button_color_query: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, create_filters!($($struct_name),*)),
        >| {
            let button_query_iter_mut = button_color_query.iter_mut();
            button_hover_change_color(button_query_iter_mut);
        }
    };
}
