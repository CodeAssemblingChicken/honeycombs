use super::board::Board;
use crate::{
    functions::rescale_board,
    parser,
    resources::{CellMeshes, GameColors, LoadState, TextSettings},
};
use bevy::{
    prelude::{Camera, Commands, Query, Res, ResMut, Transform, With},
    window::Windows,
};

pub fn setup(
    mut commands: Commands,
    wnds: Res<Windows>,
    cell_meshes: Res<CellMeshes>,
    game_colors: Res<GameColors>,
    text_settings: Res<TextSettings>,
    mut load_state: ResMut<LoadState>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if load_state.filename.is_none() {
        panic!("No level specified.");
    }
    let config = parser::board_from_file(load_state.filename.as_ref().unwrap());
    load_state.filename = None;

    let board = Board::new(
        &mut commands,
        &config,
        &text_settings,
        &cell_meshes,
        &game_colors,
        load_state.ids.unwrap(),
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
