use crate::maze::constants::ui::{BUTTON_COLOR, BUTTON_HOVER_COLOR};

use super::*;
use bevy::{prelude::*, state::state::FreelyMutableState};

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

pub(crate) fn buttons_hover_changle_color(
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
