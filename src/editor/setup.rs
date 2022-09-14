use super::{board::Board, components::CellUpdateEvent};
use crate::{
    functions::rescale_board,
    parser,
    resources::{CellMeshes, GameColors, LoadState, TextSettings},
    structs::BoardConfig,
};
use bevy::{
    prelude::{Commands, EventWriter, Res, ResMut, Transform},
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
    load_state: ResMut<LoadState>,
    mut ev_cell_update: EventWriter<CellUpdateEvent>,
) {
    let config = if let Some(filename) = load_state.filename.clone() {
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

    let mut root_transform = Transform::identity();
    for wnd in wnds.iter() {
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
        &cell_meshes,
        &game_colors,
        &text_settings,
    );

    // for wnd in wnds.iter() {
    //     rescale_board(
    //         config.width,
    //         config.height,
    //         4,
    //         wnd.width(),
    //         wnd.height(),
    //         &mut camera_query,
    //     );
    // }
    commands.insert_resource(board);
    ev_cell_update.send(CellUpdateEvent);
}
