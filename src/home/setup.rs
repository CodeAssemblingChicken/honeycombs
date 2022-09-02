use super::functions::spawn_option_cell;
use crate::{
    constants::{MED_SCALE, RADIUS, Z_INDEX_CELL_BACK, Z_INDEX_TEXT},
    functions::rescale_board,
    resources::{CellMeshes, GameColors, TextSettings},
    states::AppState,
};
use bevy::{
    math::Vec3,
    prelude::{default, AssetServer, Camera, Commands, Query, Res, Transform, With},
    sprite::SpriteBundle,
    window::Windows,
};

pub fn setup(
    mut commands: Commands,
    wnds: Res<Windows>,
    cell_meshes: Res<CellMeshes>,
    game_colors: Res<GameColors>,
    text_settings: Res<TextSettings>,
    asset_server: Res<AssetServer>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let mut big_transform =
        Transform::from_translation(Vec3::new(0., -2. * RADIUS * MED_SCALE, Z_INDEX_CELL_BACK));
    big_transform.rotate_z(f32::to_radians(90.0));
    spawn_option_cell(
        &mut commands,
        &cell_meshes,
        &game_colors,
        &text_settings,
        big_transform,
        AppState::LevelSelection,
        "Start",
    );
    big_transform.translation = Vec3::new(
        -3. * RADIUS * MED_SCALE,
        -RADIUS * MED_SCALE,
        Z_INDEX_CELL_BACK,
    );
    spawn_option_cell(
        &mut commands,
        &cell_meshes,
        &game_colors,
        &text_settings,
        big_transform,
        AppState::Editor,
        "Editor",
    );
    big_transform.translation = Vec3::new(
        3. * RADIUS * MED_SCALE,
        -RADIUS * MED_SCALE,
        Z_INDEX_CELL_BACK,
    );
    spawn_option_cell(
        &mut commands,
        &cell_meshes,
        &game_colors,
        &text_settings,
        big_transform,
        AppState::Settings,
        "Options",
    );

    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("branding/logo.png"),
        transform: Transform::from_translation(Vec3::new(
            0.,
            2. * RADIUS * MED_SCALE,
            Z_INDEX_TEXT,
        )),
        ..default()
    });

    for wnd in wnds.iter() {
        // TODO: Remove hard-coded width/height
        rescale_board(10, 6, 1, wnd.width(), wnd.height(), &mut camera_query);
    }
}
