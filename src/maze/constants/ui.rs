use bevy::{color::Color, ui::Val};

pub const ACTION_BUTTON_COLOR: Color = Color::srgb(128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0);
pub const ACTION_BUTTON_HOVER_COLOR: Color =
    Color::srgb(173.0 / 255.0, 216.0 / 255.0, 230.0 / 255.0);

pub const ACTION_BUTTON_WIDTH: Val = Val::Px(180.0);
pub const ACTION_BUTTON_HEIGHT: Val = Val::Px(50.0);

pub const SAVE_SLOT_BUTTON_COLOR: Color = Color::srgb(128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0);
pub const NOT_EMPTY_SAVE_SLOT_BUTTON_COLOR: Color = Color::srgb(255.0, 0.0, 0.0);

pub const SAVE_SLOT_BUTTON_WIDTH: Val = Val::Px(30.0);
pub const SAVE_SLOT_BUTTON_HEIGHT: Val = Val::Px(30.0);
