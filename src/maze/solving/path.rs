use bevy::prelude::*;

use crate::maze::common::{
    cell::MazeCell,
    states::{MazeState, MenuState},
};

use super::{predecessors::PredecessorsMap, EndCell};

#[derive(Event)]
pub struct DrawShortestPath;

pub fn draw_shortest_path(
    mut shortest_path_reader: EventReader<DrawShortestPath>,
    predecessors_map: Res<PredecessorsMap>,
    mut maze_state: ResMut<NextState<MazeState>>,
    end_cell_entity_query: Query<Entity, With<EndCell>>,
    mut maze_cell_sprite_query: Query<&mut Sprite, With<MazeCell>>,
) {
    let end_cell_entity = end_cell_entity_query.get_single().unwrap();

    for _ in shortest_path_reader.read() {
        let mut path = Vec::new();
        let mut current_entity = Some(end_cell_entity);

        while let Some(entity) = current_entity {
            path.push(entity);
            current_entity = predecessors_map.get(&entity).copied();
        }

        if path.len() <= 2 {
            return;
        }

        for &entity in &path[1..path.len() - 1] {
            let mut maze_cell_sprite = maze_cell_sprite_query.get_mut(entity).unwrap();
            maze_cell_sprite.color = Color::linear_rgb(0.0, 128.0, 0.0);
        }

        maze_state.set(MazeState::MainMenu(MenuState::WithSolvedMaze));
    }
}
