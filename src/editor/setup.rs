use super::{board::Board, components::CellUpdateEvent};
use crate::{
    components::BoardConfig,
    functions::rescale_board,
    parser,
    resources::{CellMeshes, GameColors, LoadState, TextSettings},
};
use bevy::{
    prelude::{Camera, Commands, EventWriter, Query, Res, ResMut, Transform, With},
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
    mut level_file: ResMut<LoadState>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    mut ev_cell_update: EventWriter<CellUpdateEvent>,
) {
    let config = if let Some(filename) = level_file.filename.clone() {
        level_file.filename = None;
        parser::board_from_file(&filename)
    } else {
        // TODO: Think about these hardcoded values
        BoardConfig {
            width: 33,
            height: 18,
            cells: vec![vec![(None, false); 33]; 18],
            hints: Vec::new(),
            text: None,
        }
    };

    let board = Board::new(
        &mut commands,
        &config,
        &cell_meshes,
        &game_colors,
        &text_settings,
    );

    for wnd in wnds.iter() {
        rescale_board(
            config.width,
            config.height,
            4,
            wnd.width(),
            wnd.height(),
            &mut camera_query,
        );
    }
    commands.insert_resource(board);
    ev_cell_update.send(CellUpdateEvent);
}
