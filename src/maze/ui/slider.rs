use bevy::{ecs::system::EntityCommands, input::mouse::MouseMotion, prelude::*};

#[derive(Component)]
pub struct Slider;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum SliderHandleState {
    Pressed,
    Released,

    #[default]
    None,
}

#[derive(Component)]
pub struct SliderHandle(SliderHandleState);

fn spawn_slider_handle(builder: &mut ChildBuilder) {
    builder.spawn((
        NodeBundle {
            style: Style {
                width: Val::Px(20.0),
                height: Val::Px(20.0),
                ..default()
            },

            background_color: Color::linear_rgb(30.0, 144.0, 255.0).into(),
            ..default()
        },
        SliderHandle(SliderHandleState::None),
        Interaction::default(),
    ));
}

pub fn spawn_slider<'a>(builder: &'a mut ChildBuilder) -> EntityCommands<'a> {
    let mut slider = builder.spawn((
        NodeBundle {
            background_color: Color::linear_rgb(255.0, 0.0, 0.0).into(),
            style: Style {
                width: Val::Px(180.0),
                height: Val::Px(10.0),
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
    buttons: Res<ButtonInput<MouseButton>>,
    slider_handle_interraction_query: Query<
        &Interaction,
        (Changed<Interaction>, With<SliderHandle>),
    >,
    mut slider_handle_query: Query<&mut SliderHandle>,
) {
    let mut slider_handle = slider_handle_query.get_single_mut().unwrap();

    let pressed = slider_handle_interraction_query
        .into_iter()
        .any(|&i| i == Interaction::Pressed);

    let left_click_pressed = buttons.pressed(MouseButton::Left);

    if pressed {
        slider_handle.0 = SliderHandleState::Pressed
    }

    if !left_click_pressed {
        slider_handle.0 = SliderHandleState::Released
    }
}

pub fn move_slider(
    mut slider_handle_query: Query<(&mut Style, &SliderHandle)>,
    mut mouse_motion_reader: EventReader<MouseMotion>,
) {
    let (mut slider_handle_style, slider_handle) = slider_handle_query.get_single_mut().unwrap();

    if slider_handle.0 == SliderHandleState::Pressed {
        for ev in mouse_motion_reader.read() {
            // let mut slider_handle_style = slider_handle_query.get_single_mut().unwrap();

            let current_position_left = match slider_handle_style.left {
                Val::Px(px) => px,
                _ => 0.0,
            };

            slider_handle_style.left = Val::Px(current_position_left + ev.delta.x);
        }
    }
}
