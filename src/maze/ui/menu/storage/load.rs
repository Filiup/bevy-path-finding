use super::*;
use std::path::Path;

use crate::maze::{
    common::{
        cell::MazeCell,
        states::{MazeState, MenuState},
        wall::MazeWall,
    },
    grid::MazeCellGrid,
    storage::read_maze,
};

#[derive(Component)]
pub struct LoadMenu;

pub fn build_menu(mut commands: Commands) {
    spawn_ui_container(&mut commands, LoadMenu)
        .with_children(|builder| {
            builder.spawn((
                TextLayout::new_with_justify(JustifyText::Center),
                Text::new("Choose slot to load maze"),
            ));
        })
        .with_children(|builder| {
            spawn_slot_container(builder).with_children(|builder| {
                for order in 1..11 {
                    let save_path = format!("saves/save_{}.mz", order);
                    let save_slot = if Path::new(&save_path).exists() {
                        StorageSlot::new(order, false)
                    } else {
                        StorageSlot::new(order, true)
                    };

                    spawn_storage_slot_button(builder, save_slot, LoadSlotButton);
                }
            });
        });
}

#[allow(clippy::type_complexity)]
pub fn load_maze(
    mut commands: Commands,
    cell_grid: Res<MazeCellGrid>,
    mut maze_state: ResMut<NextState<MazeState>>,

    maze_cell_children_query: Query<&Children, With<MazeCell>>,
    maze_wall_query: Query<&MazeWall>,
    storage_slot_interraction: Query<
        (&Interaction, &StorageSlot),
        (Changed<Interaction>, With<LoadSlotButton>),
    >,
) {
    let pressed_slot = storage_slot_interraction
        .iter()
        .find(|(&interraction, _)| interraction == Interaction::Pressed);

    if let Some((_, storage_slot)) = pressed_slot {
        if storage_slot.empty {
            return;
        }

        let maze_storage = read_maze(storage_slot.slot);

        for (position, wall_directions) in maze_storage {
            let (row, col) = position;
            let maze_cell_entity = cell_grid.get(row, col).unwrap();

            let maze_cell_childrens = maze_cell_children_query.get(maze_cell_entity).unwrap();

            for &wall_entity in maze_cell_childrens {
                let maze_wall = maze_wall_query.get(wall_entity).unwrap();

                if wall_directions.contains(&maze_wall.direction) {
                    commands.entity(wall_entity).despawn();
                }
            }
        }

        maze_state.set(MazeState::MainMenu(MenuState::WithMaze));
    }
}

pub(crate) struct LoadMenuPlugin;
impl Plugin for LoadMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MazeState::MazeLoad), build_menu)
            .add_systems(OnExit(MazeState::MazeLoad), despawn_menu::<LoadMenu>)
            .add_systems(Update, load_maze);
    }
}
