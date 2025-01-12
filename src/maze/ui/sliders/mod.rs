pub mod builders;
mod interactions;

use bevy::prelude::*;
use interactions::*;

pub const SLIDER_WIDTH: f32 = 180.0;
pub const SLIDER_HEIGHT: f32 = 10.0;

pub const SLIDER_HANDLE_WIDTH: f32 = 20.0;
pub const SLIDER_HANDLE_HEIGHT: f32 = 20.0;

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum SliderHandleState {
    Pressed(f32),

    #[default]
    Released,
}

#[derive(Component)]
pub struct SliderTrack;

#[derive(Component)]
pub struct Slider;

#[derive(Component)]
pub struct SliderText;

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum SliderDirection {
    #[default]
    Ascending,
    Descending,
}

#[derive(Component, Default)]
pub struct SliderHandle {
    state: SliderHandleState,
    pub current_value: u64,
    pub max_value: u64,
    pub direction: SliderDirection,
}

pub struct SlidersPlugin;

impl Plugin for SlidersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                change_sliders_state,
                (move_sliders, change_sliders_value, change_sliders_text)
                    .before(change_sliders_state),
            ),
        );
    }
}
