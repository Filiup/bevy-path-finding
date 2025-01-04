use std::path::Path;

use crate::maze::{
    common::{cell::MazeCell, states::MazeState, wall::MazeWall},
    constants::ui::{SAVE_SLOT_BUTTON_COLOR, SAVE_SLOT_BUTTON_HEIGHT, SAVE_SLOT_BUTTON_WIDTH},
    grid::MazeCellGrid,
    storage::read_maze,
};

use super::*;

#[derive(Component)]
pub struct LoadMenu;

#[derive(Component)]
pub struct LoadSlotButton;

#[derive(Component, Clone, Copy)]
pub struct LoadSlot {
    pub slot: usize,
}

impl LoadSlot {
    pub fn new(slot: usize) -> LoadSlot {
        LoadSlot { slot }
    }
}

fn spawn_load_slot_button<'a>(
    builder: &'a mut ChildBuilder,
    load_slot: LoadSlot,
) -> EntityCommands<'a> {
    spawn_button(
        builder,
        LoadSlotButton,
        load_slot,
        SAVE_SLOT_BUTTON_WIDTH,
        SAVE_SLOT_BUTTON_HEIGHT,
        SAVE_SLOT_BUTTON_COLOR,
        &load_slot.slot.to_string(),
    )
}

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
                for order in 0..11 {
                    let save_path = format!("saves/save_{}.mz", order);
                    if !Path::new(&save_path).exists() {
                        continue;
                    }

                    spawn_load_slot_button(builder, LoadSlot::new(order));
                }
            });
        });
}

pub fn load_maze(
    mut commands: Commands,
    maze_cell_children_query: Query<&Children, With<MazeCell>>,
    maze_wall_query: Query<&MazeWall>,
    load_slot_interraction: Query<(&Interaction, &LoadSlot), Changed<Interaction>>,
    cell_grid: Res<MazeCellGrid>,
) {
    let pressed_slot = load_slot_interraction
        .iter()
        .find(|(&interraction, _)| interraction == Interaction::Pressed);

    if let Some((_, load_slot)) = pressed_slot {
        let maze_storage = read_maze(load_slot.slot);

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
    }
}

pub struct LoadMenuPlugin;
impl Plugin for LoadMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MazeState::MazeLoad), build_menu)
            .add_systems(OnExit(MazeState::MazeLoad), despawn_menu::<LoadMenu>)
            .add_systems(Update, load_maze);
    }
}
