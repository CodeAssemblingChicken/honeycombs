use super::board::Board;
use crate::{
    functions::rescale_board,
    parser,
    resources::{CellMeshes, GameColors, LoadState, TextSettings, Viewport},
};
use bevy::{
    prelude::{Commands, Res, ResMut, Transform},
    window::Windows,
};

pub fn setup(
    mut commands: Commands,
    wnds: Res<Windows>,
    (cell_meshes, game_colors, text_settings): (
        Res<CellMeshes>,
        Res<GameColors>,
        Res<TextSettings>,
    ),
    mut load_state: ResMut<LoadState>,
    mut viewport: ResMut<Viewport>,
) {
    if load_state.filename.is_none() {
        panic!("No level specified.");
    }
    let config = parser::board_from_file(load_state.filename.as_ref().unwrap());
    load_state.filename = None;

    let mut root_transform = Transform::identity();
    for wnd in wnds.iter() {
        // TODO: Remove hard-coded width/height
        rescale_board(
            config.width,
            config.height,
            4,
            wnd.width(),
            wnd.height(),
            &mut root_transform,
        );
    }
    let board = Board::new(
        &mut commands,
        root_transform,
        &config,
        (&cell_meshes, &game_colors, &text_settings),
        &mut viewport,
        load_state.ids.unwrap(),
    );

    commands.insert_resource(board);
}
