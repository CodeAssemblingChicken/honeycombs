mod board;
mod components;
mod functions;
mod parser;
pub mod resources;
mod systems;

use self::{board::Board, functions::rescale_board, resources::LevelFile, systems::*};
use crate::{
    resources::{CellColors, TextSettings},
    states::AppState,
};
use bevy::{
    app::App,
    prelude::{
        Assets, Camera, Commands, Mesh, ParallelSystemDescriptorCoercion, Query, Res, ResMut,
        SystemSet, Transform, With,
    },
    window::Windows,
};

const STATE: AppState = AppState::Level;

pub fn prepare_level(app: &mut App) {
    app.add_system_set(SystemSet::on_enter(STATE).with_system(setup))
        .add_system_set(
            SystemSet::on_update(STATE)
                .with_system(mouse_over_cell)
                .with_system(mouse_enter_cell.before(mouse_over_cell))
                .with_system(mouse_exit_cell.before(mouse_enter_cell))
                .with_system(
                    mouse_click_cell
                        .after(mouse_enter_cell)
                        .after(mouse_exit_cell),
                )
                .with_system(window_resize_system)
                .with_system(check_solved),
        )
        .add_system_set(SystemSet::on_exit(STATE).with_system(cleanup));
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    cell_colors: ResMut<CellColors>,
    text_settings: ResMut<TextSettings>,
    mut level_file: ResMut<LevelFile>,
    wnds: Res<Windows>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if level_file.filename.is_none() {
        panic!("No level specified.");
    }
    let cells = parser::board_from_file(level_file.filename.as_ref().unwrap());
    level_file.filename = None;

    let b = Board::new(
        &mut commands,
        meshes,
        cells,
        &text_settings,
        cell_colors.white.clone(),
        (
            cell_colors.yellow_medium.clone(),
            cell_colors.yellow_light.clone(),
        ),
        (
            cell_colors.gray_medium.clone(),
            cell_colors.gray_light.clone(),
        ),
        (
            cell_colors.blue_medium.clone(),
            cell_colors.blue_light.clone(),
        ),
    );
    for w in wnds.iter() {
        rescale_board(&b, w.width(), w.height(), &mut camera_query);
    }
    commands.insert_resource(b);
}

fn cleanup(mut commands: Commands, board: Res<Board>) {
    board.despawn_all(&mut commands);
}
