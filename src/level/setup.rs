use super::board::Board;
use crate::{
    functions::rescale_board,
    parser,
    resources::{CellMeshes, GameColors, LoadState, Locale, TextSettings},
};
use bevy::{
    prelude::{Assets, Commands, Mesh, Res, ResMut, Transform},
    sprite::ColorMaterial,
    window::Windows,
};

pub fn setup(
    mut commands: Commands,
    wnds: Res<Windows>,
    (cell_meshes, game_colors, locale, text_settings): (
        Res<CellMeshes>,
        Res<GameColors>,
        Res<Locale>,
        Res<TextSettings>,
    ),
    load_state: ResMut<LoadState>,
    (mut meshes, mut colors): (ResMut<Assets<Mesh>>, ResMut<Assets<ColorMaterial>>),
) {
    if load_state.filename.is_none() {
        panic!("No level specified.");
    }
    let config = parser::board_from_file(load_state.filename.as_ref().unwrap(), &locale);

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
        (&cell_meshes, &game_colors, &text_settings),
        load_state.ids.unwrap(),
        (&mut meshes, &mut colors),
    );

    commands.insert_resource(board);
}
