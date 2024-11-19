use bevy::{
    prelude::{Changed, Component, NextState, Query, ResMut, States, With},
    state::state::FreelyMutableState,
    ui::Interaction,
};

pub fn button_interaction_system<T, S>(
    next_state: S,
    interaction_type: Interaction,
) -> impl Fn(Query<'_, '_, &Interaction, (Changed<Interaction>, With<T>)>, ResMut<'_, NextState<S>>)
where
    T: Component,
    S: States + FreelyMutableState + Copy,
{
    move |button_query: Query<&Interaction, (Changed<Interaction>, With<T>)>,
          mut next_state_res: ResMut<NextState<S>>| {
        for interaction in button_query.iter() {
            if interaction_type == *interaction {
                next_state_res.set(next_state);
            }
        }
    }
}
