use super::*;
use bevy::{prelude::*, window::PrimaryWindow};

pub fn change_sliders_text(
    slider_track_parent_query: Query<&Parent, With<SliderTrack>>,

    mut slider_text_query: Query<(&mut Text, &Parent), With<SliderText>>,
    slider_handle_query: Query<(&SliderHandle, &Parent)>,
) {
    for (slider_handle, slider_handle_parent) in slider_handle_query.into_iter() {
        let is_pressed = matches!(slider_handle.state, SliderHandleState::Pressed(_));
        if !is_pressed {
            continue;
        }

        let slider_track_parent = slider_track_parent_query
            .get(slider_handle_parent.get())
            .unwrap();

        let slider_text = slider_text_query
            .iter_mut()
            .find(|(_, parent)| parent.get() == slider_track_parent.get());

        if let Some((mut text, _)) = slider_text {
            text.0 = slider_handle.current_value.to_string();
        }
    }
}

pub fn change_sliders_state(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut slider_handle_query: Query<(&mut SliderHandle, &Node, &Interaction)>,
) {
    let primary_window = window_query.single();

    for (mut slider_handle, node, interraction) in slider_handle_query.iter_mut() {
        let pressed = *interraction == Interaction::Pressed;
        let left_click_pressed = mouse_buttons.pressed(MouseButton::Left);

        if pressed {
            if let Some(cursor_position) = primary_window.cursor_position() {
                let current_left = match node.left {
                    Val::Px(value) => value,
                    _ => 0.0,
                };

                slider_handle.state = SliderHandleState::Pressed(cursor_position.x - current_left);
            }
        }

        if !left_click_pressed {
            slider_handle.state = SliderHandleState::Released;
        }
    }
}

pub fn change_sliders_value(mut slider_handle_query: Query<(&Node, &mut SliderHandle)>) {
    for (slider_handle_node, mut slider_handle) in slider_handle_query.iter_mut() {
        if let SliderHandleState::Pressed(_) = slider_handle.state {
            let current_left = match slider_handle_node.left {
                Val::Px(value) => value,
                _ => 0.0,
            };

            let max_position = SLIDER_WIDTH - SLIDER_HANDLE_WIDTH;
            let normalized_value = current_left / max_position;

            slider_handle.current_value = match slider_handle.direction {
                SliderDirection::Ascending => {
                    (normalized_value * slider_handle.max_value as f32) as u64
                }
                SliderDirection::Descending => {
                    ((1.0 - normalized_value) * slider_handle.max_value as f32) as u64
                }
            }
        }
    }
}

pub fn move_sliders(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut slider_handle_query: Query<(&mut Node, &SliderHandle)>,
) {
    let primary_window = window_query.single();

    for (mut slider_handle_node, slider_handle) in slider_handle_query.iter_mut() {
        if let SliderHandleState::Pressed(start_pressed_x) = slider_handle.state {
            if let Some(cursor_position) = primary_window.cursor_position() {
                let new_left = (cursor_position.x - start_pressed_x)
                    .clamp(0.0, SLIDER_WIDTH - SLIDER_HANDLE_WIDTH);

                slider_handle_node.left = Val::Px(new_left);
            }
        }
    }
}
