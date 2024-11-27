use bevy::{ecs::system::EntityCommands, prelude::*, window::PrimaryWindow};

pub const SLIDER_WIDTH: f32 = 180.0;
pub const SLIDER_HEIGHT: f32 = 10.0;

pub const SLIDER_HANDLE_WIDTH: f32 = 20.0;
pub const SLIDER_HANDLE_HEIGHT: f32 = 20.0;

#[derive(Component)]
pub struct Slider;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SliderHandleState {
    Pressed(f32),
    
    #[default]
    Released,
}

#[derive(Component)]
pub struct SliderHandle(SliderHandleState);

fn spawn_slider_handle(builder: &mut ChildBuilder) {
    builder.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(SLIDER_HANDLE_WIDTH),
                height: Val::Px(SLIDER_HANDLE_HEIGHT),
                ..default()
            },

            background_color: Color::linear_rgb(30.0, 144.0, 255.0).into(),
            ..default()
        },
        SliderHandle(SliderHandleState::default()),
        Interaction::default(),
    ));
}

pub fn spawn_slider<'a>(builder: &'a mut ChildBuilder) -> EntityCommands<'a> {
    let mut slider = builder.spawn((
        NodeBundle {
            background_color: Color::linear_rgb(255.0, 0.0, 0.0).into(),
            style: Style {
                width: Val::Px(SLIDER_WIDTH),
                height: Val::Px(SLIDER_HEIGHT),
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        },
        Slider,
    ));

    slider.with_children(spawn_slider_handle);

    slider
}

pub fn change_slider_state(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut slider_handle_query: Query<(&mut SliderHandle, &Style)>,
    slider_handle_interraction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<SliderHandle>),
    >,
) {
    let primary_window = window_query.single();
    let (mut slider_handle, style) = slider_handle_query.single_mut();

    let pressed = slider_handle_interraction_query
        .into_iter()
        .any(|&i| i == Interaction::Pressed);
    let left_click_pressed = mouse_buttons.pressed(MouseButton::Left);

    if pressed {
        if let Some(cursor_position) = primary_window.cursor_position() {
            let current_left = match style.left {
                Val::Px(value) => value,
                _ => 0.0,
            };

            slider_handle.0 = SliderHandleState::Pressed(cursor_position.x - current_left);
        }
    }
    if !left_click_pressed {
        slider_handle.0 = SliderHandleState::Released;
    }
}

pub fn move_slider(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut slider_handle_query: Query<(&mut Style, &SliderHandle)>,
) {
    let (mut slider_handle_style, slider_handle) = slider_handle_query.get_single_mut().unwrap();
    let primary_window = window_query.single();

    if let SliderHandleState::Pressed(start_pressed_x) = slider_handle.0 {
        if let Some(cursor_position) = primary_window.cursor_position() {
            let new_left = (cursor_position.x - start_pressed_x)
                .clamp(0.0, SLIDER_WIDTH - SLIDER_HANDLE_WIDTH);

            slider_handle_style.left = Val::Px(new_left);
        }
    }
}
