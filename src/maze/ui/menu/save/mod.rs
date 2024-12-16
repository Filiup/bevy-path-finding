use bevy::prelude::*;

use super::{despawn_menu, main::spawn_button, spawn_ui_container};
use crate::maze::{common::states::MazeState, storage::SaveMazeEvent};

pub const SAVE_MAZE_BUTTON_COLOR: Color = Color::srgb(128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0);
#[derive(Component)]
pub struct SaveMenu;

#[derive(Component)]
pub struct SaveSlotButton;

#[derive(Component, Clone, Copy)]
pub struct SaveSlot {
    slot: usize,
}

impl SaveSlot {
    pub fn new(order: usize) -> SaveSlot {
        SaveSlot { slot: order }
    }
}

pub fn save_maze(
    maze_slot_interraction_query: Query<(&Interaction, &SaveSlot), Changed<Interaction>>,
    mut save_maze_writer: EventWriter<SaveMazeEvent>,
) {
    let pressed_slot = maze_slot_interraction_query
        .iter()
        .find(|(&interraction, _)| interraction == Interaction::Pressed);

    if let Some((_, save_slot)) = pressed_slot {
        save_maze_writer.send(SaveMazeEvent {
            slot: save_slot.slot,
        });
    }
}

fn spawn_save_container<'a>(builder: &'a mut ChildBuilder) -> EntityCommands<'a> {
    let container = builder.spawn(Node {
        display: Display::Grid,
        grid_template_columns: RepeatedGridTrack::flex(5, 1.0),
        grid_template_rows: RepeatedGridTrack::flex(2, 1.0),
        row_gap: Val::Px(10.0),
        column_gap: Val::Px(10.0),
        ..default()
    });

    container
}

fn spawn_save_slot_button<'a>(
    builder: &'a mut ChildBuilder,
    save_slot: SaveSlot,
) -> EntityCommands<'a> {
    let save_button_width = Val::Px(30.0);
    let save_button_height = Val::Px(30.0);
    spawn_button(
        builder,
        SaveSlotButton,
        save_slot,
        save_button_width,
        save_button_height,
        SAVE_MAZE_BUTTON_COLOR,
        &save_slot.slot.to_string(),
    )
}

pub fn build_menu(mut commands: Commands) {
    spawn_ui_container(&mut commands, SaveMenu)
        .with_children(|builder| {
            builder.spawn((
                Node {
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                Text::new("Select save location"),
            ));
        })
        .with_children(|builder| {
            spawn_save_container(builder).with_children(|builder| {
                for order in 1..11 {
                    spawn_save_slot_button(builder, SaveSlot::new(order));
                }
            });
        });
}

pub struct SaveMenuPlugin;

impl Plugin for SaveMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MazeState::MazeSave), build_menu)
            .add_systems(OnExit(MazeState::MazeSave), despawn_menu::<SaveMenu>)
            .add_systems(Update, save_maze);
    }
}
