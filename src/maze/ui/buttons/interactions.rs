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
