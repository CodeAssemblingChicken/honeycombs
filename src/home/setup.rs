use super::functions::spawn_option_cell;
use crate::{
    assets::LocaleAsset,
    components::RootComponent,
    constants::{MED_SCALE, RADIUS, Z_INDEX_CELL_BACK, Z_INDEX_TEXT},
    functions::rescale_board,
    resources::{CellMeshes, GameColors, LocaleAssets, Profile, TextSettings},
    states::AppState,
};
use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{default, AssetServer, Assets, Commands, Res, SpatialBundle, Transform},
    sprite::SpriteBundle,
    window::Windows,
};

type StandardResources<'a> = (
    Res<'a, CellMeshes>,
    Res<'a, GameColors>,
    Res<'a, LocaleAssets>,
    Res<'a, Profile>,
    Res<'a, TextSettings>,
);
pub fn setup(
    mut commands: Commands,
    wnds: Res<Windows>,
    (cell_meshes, game_colors, locale, profile, text_settings): StandardResources,
    asset_server: Res<AssetServer>,
    locales: Res<Assets<LocaleAsset>>,
) {
    let mut big_transform = Transform::from_xyz(0., 0., Z_INDEX_CELL_BACK);
    big_transform.rotate_z(f32::to_radians(90.0));
    let start_cell = spawn_option_cell(
        &mut commands,
        &cell_meshes,
        &game_colors,
        &text_settings,
        big_transform,
        AppState::LevelSelection,
        locale
            .get_string("start", &locales, &profile)
            .unwrap_or(&"String not found".to_string()),
    );
    big_transform.translation = Vec3::new(
        -1.2 * RADIUS * MED_SCALE,
        -2. * RADIUS * MED_SCALE,
        Z_INDEX_CELL_BACK,
    );
    let editor_cell = spawn_option_cell(
        &mut commands,
        &cell_meshes,
        &game_colors,
        &text_settings,
        big_transform.with_scale(Vec3::new(0.75, 0.75, 0.75)),
        AppState::Editor,
        locale
            .get_string("editor", &locales, &profile)
            .unwrap_or(&"String not found".to_string()),
    );
    big_transform.translation = Vec3::new(
        3. * RADIUS * MED_SCALE,
        -RADIUS * MED_SCALE,
        Z_INDEX_CELL_BACK,
    );
    let quit_cell = spawn_option_cell(
        &mut commands,
        &cell_meshes,
        &game_colors,
        &text_settings,
        big_transform.with_scale(Vec3::new(0.75, 0.75, 0.75)),
        AppState::Quit,
        locale
            .get_string("quit", &locales, &profile)
            .unwrap_or(&"String not found".to_string()),
    );
    big_transform.translation = Vec3::new(
        -3. * RADIUS * MED_SCALE,
        -RADIUS * MED_SCALE,
        Z_INDEX_CELL_BACK,
    );
    let options_cell = spawn_option_cell(
        &mut commands,
        &cell_meshes,
        &game_colors,
        &text_settings,
        big_transform.with_scale(Vec3::new(0.75, 0.75, 0.75)),
        AppState::Options,
        locale
            .get_string("options", &locales, &profile)
            .unwrap_or(&"String not found".to_string()),
    );
    big_transform.translation = Vec3::new(
        1.2 * RADIUS * MED_SCALE,
        -2. * RADIUS * MED_SCALE,
        Z_INDEX_CELL_BACK,
    );
    let credits_cell = spawn_option_cell(
        &mut commands,
        &cell_meshes,
        &game_colors,
        &text_settings,
        big_transform.with_scale(Vec3::new(0.75, 0.75, 0.75)),
        AppState::Credits,
        locale
            .get_string("credits", &locales, &profile)
            .unwrap_or(&"String not found".to_string()),
    );

    let logo_entity = commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("branding/logo.png"),
            transform: Transform::from_xyz(0., 2. * RADIUS * MED_SCALE, Z_INDEX_TEXT),
            ..default()
        })
        .id();

    let mut root_transform = Transform::identity();
    for wnd in wnds.iter() {
        // TODO: Remove hard-coded width/height
        rescale_board(10, 6, 1, wnd.width(), wnd.height(), &mut root_transform);
    }

    commands
        .spawn()
        .push_children(&[
            start_cell,
            editor_cell,
            quit_cell,
            options_cell,
            credits_cell,
            logo_entity,
        ])
        .insert_bundle(SpatialBundle::from_transform(root_transform))
        .insert(RootComponent);
}
