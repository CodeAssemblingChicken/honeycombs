use super::board::Board;
use crate::{
    functions::rescale_board,
    parser,
    resources::{CellColors, CellMeshes, LevelFile, TextSettings},
};
use bevy::{
    prelude::{Camera, Commands, Query, Res, ResMut, Transform, With},
    window::Windows,
};

pub fn setup(
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
    let config = parser::board_from_file(level_file.filename.as_ref().unwrap());
    level_file.filename = None;

    let board = Board::new(
        &mut commands,
        &config,
        &text_settings,
        &cell_meshes,
        &cell_colors,
    );

    for wnd in wnds.iter() {
        rescale_board(
            board.width,
            board.height,
            4,
            wnd.width(),
            wnd.height(),
            &mut camera_query,
        );
    }
    commands.insert_resource(board);
}
