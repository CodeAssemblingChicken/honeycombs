mod board;
mod components;
mod parser;
pub mod resources;
mod systems;

use self::{board::Board, resources::LevelFile, systems::*};
use crate::{
    cleanup,
    functions::rescale_board,
    resources::{CellColors, CellMeshes, TextSettings},
    states::AppState,
};
use bevy::{
    app::App,
    prelude::{
        Camera, Commands, ParallelSystemDescriptorCoercion, Query, Res, ResMut, SystemSet,
        Transform, With,
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
    wnds: Res<Windows>,
    cell_meshes: Res<CellMeshes>,
    cell_colors: Res<CellColors>,
    text_settings: Res<TextSettings>,
    mut level_file: ResMut<LevelFile>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if level_file.filename.is_none() {
        panic!("No level specified.");
    }
    let cells = parser::board_from_file(level_file.filename.as_ref().unwrap());
    level_file.filename = None;

    let b = Board::new(
        &mut commands,
        cells,
        &text_settings,
        &cell_meshes,
        &cell_colors,
    );
    for wnd in wnds.iter() {
        rescale_board(
            b.width,
            b.height,
            4,
            wnd.width(),
            wnd.height(),
            &mut camera_query,
        );
    }
    commands.insert_resource(b);
}
