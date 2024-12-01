use bevy::{ecs::system::EntityCommands, prelude::*, window::PrimaryWindow};

pub const SLIDER_WIDTH: f32 = 180.0;
pub const SLIDER_HEIGHT: f32 = 10.0;

pub const SLIDER_HANDLE_WIDTH: f32 = 20.0;
pub const SLIDER_HANDLE_HEIGHT: f32 = 20.0;

pub const SLIDER_START_VALUE: i32 = 1000;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SliderHandleState {
    Pressed(f32),

    #[default]
    Released,
}

#[derive(Component)]
pub struct Slider;

#[derive(Component)]
pub struct SliderText;

#[derive(Component)]
pub struct SliderHandle {
    state: SliderHandleState,
    pub value: i32,
}

impl Default for SliderHandle {
    fn default() -> Self {
        Self {
            value: SLIDER_START_VALUE,
            state: SliderHandleState::default(),
        }
    }
}

fn spawn_slider_handle(builder: &mut ChildBuilder, slider_handle_marker: impl Component) {
    builder.spawn((
        BackgroundColor(Color::linear_rgb(30.0, 144.0, 255.0)),
        Node {
            width: Val::Px(SLIDER_HANDLE_WIDTH),
            height: Val::Px(SLIDER_HANDLE_HEIGHT),
            ..default()
        },
        SliderHandle::default(),
        Interaction::default(),
        slider_handle_marker,
    ));
}

pub fn spawn_slider_text<'a>(builder: &'a mut ChildBuilder) -> EntityCommands<'a> {
    builder.spawn((Text::new(SLIDER_START_VALUE.to_string()), SliderText))
}

pub fn spawn_slider<'a>(
    builder: &'a mut ChildBuilder,
    slider_handle_marker: impl Component,
) -> EntityCommands<'a> {
    let mut slider = builder.spawn((
        BackgroundColor(Color::linear_rgb(255.0, 0.0, 0.0)),
        Node {
            width: Val::Px(SLIDER_WIDTH),
            height: Val::Px(SLIDER_HEIGHT),
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Center,

            ..default()
        },
        Slider,
    ));

    slider.with_children(|builder| spawn_slider_handle(builder, slider_handle_marker));

    slider
}

pub fn change_slider_text(
    slider_handle_query: Query<&SliderHandle, Changed<SliderHandle>>,
    mut sliter_text_query: Query<&mut Text, With<SliderText>>,
) {
    let slider_handle = slider_handle_query.get_single();
    if let Ok(slider_handle) = slider_handle {
        let mut slider_text = sliter_text_query.single_mut();
        slider_text.0 = slider_handle.value.to_string();
    }
}

pub fn change_sliders_state(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut slider_handle_query: Query<(&mut SliderHandle, &Node)>,
    slider_handle_interraction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<SliderHandle>),
    >,
) {
    let primary_window = window_query.single();
    // let (mut slider_handle, node) = slider_handle_query.single_mut();

    for (mut slider_handle, node) in slider_handle_query.iter_mut() {
        let pressed = slider_handle_interraction_query
            .into_iter()
            .any(|&i| i == Interaction::Pressed);

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

pub fn change_slider_value(mut slider_handle_query: Query<(&Node, &mut SliderHandle)>) {
    let (slider_handle_node, mut slider_handle) = slider_handle_query.single_mut();

    if let SliderHandleState::Pressed(_) = slider_handle.state {
        let current_left = match slider_handle_node.left {
            Val::Px(value) => value,
            _ => 0.0,
        };

        let max_position = SLIDER_WIDTH - SLIDER_HANDLE_WIDTH;
        let normalized_value = current_left / max_position;

        slider_handle.value = ((1.0 - normalized_value) * SLIDER_START_VALUE as f32) as i32;

        println!("{}", slider_handle.value);
    }
}

pub fn move_slider(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut slider_handle_query: Query<(&mut Node, &SliderHandle)>,
) {
    let (mut slider_handle_node, slider_handle) = slider_handle_query.get_single_mut().unwrap();
    let primary_window = window_query.single();

    if let SliderHandleState::Pressed(start_pressed_x) = slider_handle.state {
        if let Some(cursor_position) = primary_window.cursor_position() {
            let new_left = (cursor_position.x - start_pressed_x)
                .clamp(0.0, SLIDER_WIDTH - SLIDER_HANDLE_WIDTH);

            slider_handle_node.left = Val::Px(new_left);
        }
    }
}
