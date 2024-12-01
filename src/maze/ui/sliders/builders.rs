use bevy::prelude::*;

use super::*;

fn spawn_slider_handle(
    builder: &mut ChildBuilder,
    slider_handle_marker: impl Component,
    slider_handle_max_value: u64,
    slider_handle_drirection: SliderDirection,
) {
    let current_value = match slider_handle_drirection {
        SliderDirection::Ascending => 0,
        SliderDirection::Descending => slider_handle_max_value,
    };

    builder.spawn((
        BackgroundColor(Color::linear_rgb(30.0, 144.0, 255.0)),
        Node {
            width: Val::Px(SLIDER_HANDLE_WIDTH),
            height: Val::Px(SLIDER_HANDLE_HEIGHT),
            ..default()
        },
        SliderHandle {
            max_value: slider_handle_max_value,
            direction: slider_handle_drirection,
            current_value,
            ..default()
        },
        Interaction::default(),
        slider_handle_marker,
    ));
}

fn spawn_slider_text<'a>(
    builder: &'a mut ChildBuilder,
    slider_handle_max_value: u64,
    slider_handle_drirection: SliderDirection,
) -> EntityCommands<'a> {
    let value = match slider_handle_drirection {
        SliderDirection::Ascending => 0,
        SliderDirection::Descending => slider_handle_max_value,
    };

    builder.spawn((Text::new(value.to_string()), SliderText))
}

fn spawn_slider_track<'a>(
    builder: &'a mut ChildBuilder,
    slider_handle_marker: impl Component,
    slider_handle_max_value: u64,
    slider_handle_drirection: SliderDirection,
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
        SliderTrack,
    ));

    slider.with_children(|builder| {
        spawn_slider_handle(
            builder,
            slider_handle_marker,
            slider_handle_max_value,
            slider_handle_drirection,
        )
    });

    slider
}

pub fn spawn_slider<'a>(
    builder: &'a mut ChildBuilder,
    slider_handle_max_value: u64,
    slider_handle_drirection: SliderDirection,
    slider_handle_marker: impl Component,
) -> EntityCommands<'a> {
    let mut slider_container = builder.spawn((
        Node {
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(10.0),
            ..default()
        },
        Slider,
    ));

    slider_container.with_children(|builder| {
        spawn_slider_track(
            builder,
            slider_handle_marker,
            slider_handle_max_value,
            slider_handle_drirection,
        );
    });

    slider_container.with_children(|builder| {
        spawn_slider_text(builder, slider_handle_max_value, slider_handle_drirection);
    });

    slider_container
}
