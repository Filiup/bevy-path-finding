mod end_select;
mod solving;
mod start_select;

use bevy::window::PrimaryWindow;
use end_select::EndSelectMenuPlugin;
use solving::SolvingMenuPlugin;
use start_select::StartSelectMenuPlugin;

use crate::maze::{
    common::{
        cell::{CellPathCompoment, EndCell, MazeCell, StartCell},
        states::MazeState,
    },
    constants::{
        grid::{BLOCK_SIZE, CELL_COLOR},
        window::GRID_WINDOW_HEIGHT,
    },
    grid::MazeCellGrid,
};

pub(crate) use super::*;

pub struct SolveMenuPlugin;

#[allow(clippy::type_complexity)]
pub fn hover_cells_system(
    cell_path_color: Color,
) -> impl Fn(
    Query<&Window, With<PrimaryWindow>>,
    Query<&mut Sprite, With<MazeCell>>,
    Query<Entity, With<StartCell>>,
    Query<Entity, With<EndCell>>,
    Local<Option<Entity>>,
    Res<MazeCellGrid>,
) {
    move |window_query,
          mut cell_sprite_query,
          start_cell_entity_query,
          end_cell_entity_query,
          mut last_hovered_cell_entity,
          maze_grid| {
        let start_cell_entity = start_cell_entity_query
            .get_single()
            .unwrap_or(Entity::PLACEHOLDER);

        let end_cell_entity = end_cell_entity_query
            .get_single()
            .unwrap_or(Entity::PLACEHOLDER);

        let window = window_query.get_single().unwrap();
        let cursor_position = window.cursor_position();

        let mut set_color = |entity, color| {
            let cell_sprite = cell_sprite_query.get_mut(entity);
            if let Ok(mut cell_sprite) = cell_sprite {
                cell_sprite.color = color;
            }
        };

        match (cursor_position, *last_hovered_cell_entity) {
            (Some(position), _) => {
                let col = (position.x / BLOCK_SIZE).floor() as usize;
                let row = ((GRID_WINDOW_HEIGHT - position.y) / BLOCK_SIZE).floor() as usize;

                let cell_entity = maze_grid
                    .get(row, col)
                    .filter(|&e| e != start_cell_entity && e != end_cell_entity);

                match (cell_entity, *last_hovered_cell_entity) {
                    (Some(entity), _) => {
                        match *last_hovered_cell_entity {
                            Some(last_cell_entity) if last_cell_entity != entity => {
                                set_color(last_cell_entity, CELL_COLOR);
                            }
                            _ => {}
                        }

                        set_color(entity, cell_path_color);
                        *last_hovered_cell_entity = Some(entity);
                    }

                    (None, Some(last_cell_entity)) => {
                        set_color(last_cell_entity, CELL_COLOR);
                    }
                    _ => {}
                }
            }
            (None, Some(last_cell_entity)) => {
                set_color(last_cell_entity, CELL_COLOR);
            }
            _ => {}
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn set_cell_path_system(
    cell_path_component: impl CellPathCompoment,
    next_state: MazeState,
    cell_path_color: Color,
) -> impl Fn(
    Commands,
    Res<ButtonInput<MouseButton>>,
    Res<MazeCellGrid>,
    ResMut<NextState<MazeState>>,
    Query<&Window, With<PrimaryWindow>>,
    Query<&mut Sprite, With<MazeCell>>,
    Query<Entity, With<StartCell>>,
    Query<Entity, With<EndCell>>,
) {
    move |mut commands,
          mouse_input,
          maze_grid,
          mut maze_state,
          window_query,
          mut cell_sprite_query,
          start_cell_entity_query,
          end_cell_entity_query| {
        let start_cell_entity = start_cell_entity_query
            .get_single()
            .unwrap_or(Entity::PLACEHOLDER);

        let end_cell_entity = end_cell_entity_query
            .get_single()
            .unwrap_or(Entity::PLACEHOLDER);

        let window = window_query.get_single().unwrap();
        let cursor_position = window.cursor_position();

        if mouse_input.just_pressed(MouseButton::Left) && cursor_position.is_some() {
            let position = cursor_position.unwrap();
            let col = (position.x / BLOCK_SIZE).floor() as usize;
            let row = ((GRID_WINDOW_HEIGHT - position.y) / BLOCK_SIZE).floor() as usize;
            let cell_entity = maze_grid
                .get(row, col)
                .filter(|&e| e != start_cell_entity && e != end_cell_entity);

            if let Some(entity) = cell_entity {
                let mut cell_sprite = cell_sprite_query.get_mut(entity).unwrap();
                cell_sprite.color = cell_path_color;
                commands.entity(entity).insert(cell_path_component);
                maze_state.set(next_state);
            }
        }
    }
}

impl Plugin for SolveMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StartSelectMenuPlugin)
            .add_plugins(EndSelectMenuPlugin)
            .add_plugins(SolvingMenuPlugin);
    }
}
