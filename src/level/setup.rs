use super::board::Board;
use crate::{
    assets::LocaleAsset,
    functions::rescale_board,
    parser,
    resources::{CellMeshes, GameColors, LoadState, LocaleAssets, Profile, TextSettings},
};
use bevy::{
    prelude::{Assets, Commands, Mesh, Res, ResMut, Transform},
    sprite::ColorMaterial,
    window::Windows,
};

type StandardResources<'a> = (
    Res<'a, CellMeshes>,
    Res<'a, GameColors>,
    Res<'a, LocaleAssets>,
    Res<'a, Profile>,
    Res<'a, TextSettings>,
);
type StandardAssets<'a> = (
    ResMut<'a, Assets<Mesh>>,
    ResMut<'a, Assets<ColorMaterial>>,
    Res<'a, Assets<LocaleAsset>>,
);
pub fn setup(
    mut commands: Commands,
    wnds: Res<Windows>,
    (cell_meshes, game_colors, locale, profile, text_settings): StandardResources,
    load_state: ResMut<LoadState>,
    (mut meshes, mut colors, locales): StandardAssets,
) {
    if load_state.filename.is_none() {
        panic!("No level specified.");
    }
    let config = parser::board_from_file(load_state.filename.as_ref().unwrap());

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
        (
            &cell_meshes,
            &game_colors,
            &locale,
            &profile,
            &text_settings,
        ),
        load_state.ids.unwrap(),
        (&mut meshes, &mut colors, &locales),
    );

    commands.insert_resource(board);
}
