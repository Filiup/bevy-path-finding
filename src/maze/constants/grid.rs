use bevy::color::Color;

pub const BLOCK_SIZE: f32 = 40.0;
pub const WALL_HEIGHT: f32 = 2.0;

pub const CELL_COLOR: Color = Color::WHITE;
pub const WALL_COLOR: Color = Color::BLACK;

pub const START_CELL_COLOR: Color = Color::linear_rgb(0.0, 0.0, 255.0);
pub const END_CELL_COLOR: Color = Color::linear_rgb(255.0, 0.0, 0.0);
