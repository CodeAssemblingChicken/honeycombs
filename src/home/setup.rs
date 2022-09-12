use super::{components::LangSelector, functions::spawn_option_cell};
use crate::{
    assets::LocaleAsset,
    components::{Language, RootComponent},
    constants::{MED_SCALE, RADIUS, Z_INDEX_CELL_BACK, Z_INDEX_TEXT},
    functions::rescale_board,
    resources::{CellMeshes, GameColors, LocaleAssets, Profile, TextSettings},
    states::AppState,
};
use bevy::{
    hierarchy::BuildChildren,
    math::{Vec2, Vec3},
    prelude::{
        default, AssetServer, Assets, Color, Commands, Mesh, Res, ResMut, SpatialBundle, Transform,
    },
    sprite::{ColorMaterial, ColorMesh2dBundle, SpriteBundle},
    window::Windows,
};
use interactable::{components::Interactable, shapes::Shape};

pub fn setup(
    mut commands: Commands,
    wnds: Res<Windows>,
    (cell_meshes, game_colors, locale, profile, text_settings): (
        Res<CellMeshes>,
        Res<GameColors>,
        Res<LocaleAssets>,
        Res<Profile>,
        Res<TextSettings>,
    ),
    asset_server: Res<AssetServer>,
    (mut meshes, mut colors, locales): (
        ResMut<Assets<Mesh>>,
        ResMut<Assets<ColorMaterial>>,
        Res<Assets<LocaleAsset>>,
    ),
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
        -3. * RADIUS * MED_SCALE,
        -1.5 * RADIUS * MED_SCALE,
        Z_INDEX_CELL_BACK,
    );
    let editor_cell = spawn_option_cell(
        &mut commands,
        &cell_meshes,
        &game_colors,
        &text_settings,
        big_transform,
        AppState::Editor,
        locale
            .get_string("editor", &locales, &profile)
            .unwrap_or(&"String not found".to_string()),
    );
    big_transform.translation = Vec3::new(
        3. * RADIUS * MED_SCALE,
        -1.5 * RADIUS * MED_SCALE,
        Z_INDEX_CELL_BACK,
    );
    let settings_cell = spawn_option_cell(
        &mut commands,
        &cell_meshes,
        &game_colors,
        &text_settings,
        big_transform,
        AppState::Settings,
        locale
            .get_string("quit", &locales, &profile)
            .unwrap_or(&"String not found".to_string()),
    );

    let logo_entity = commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("branding/logo.png"),
            transform: Transform::from_xyz(0., 2. * RADIUS * MED_SCALE, Z_INDEX_TEXT),
            ..default()
        })
        .id();

    let pos_en = (-140.0, -2. * RADIUS * MED_SCALE + 0.8 * RADIUS);
    let pos_de = (140.0, -2. * RADIUS * MED_SCALE + 0.8 * RADIUS);
    let pos_fr = (-140.0, -2. * RADIUS * MED_SCALE - 0.8 * RADIUS);
    let pos_es = (140.0, -2. * RADIUS * MED_SCALE - 0.8 * RADIUS);
    let lang_en = commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("lang/en.png"),
            transform: Transform::from_xyz(pos_en.0, pos_en.1, Z_INDEX_TEXT),
            ..default()
        })
        .insert(Interactable {
            shape: Shape::Quad(interactable::shapes::Quad {
                width: 200.,
                height: 120.,
            }),
            ..default()
        })
        .insert(Language::EN)
        .id();
    let lang_de = commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("lang/de.png"),
            transform: Transform::from_xyz(pos_de.0, pos_de.1, Z_INDEX_TEXT),
            ..default()
        })
        .insert(Interactable {
            shape: Shape::Quad(interactable::shapes::Quad {
                width: 200.,
                height: 120.,
            }),
            ..default()
        })
        .insert(Language::DE)
        .id();
    let lang_fr = commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("lang/fr.png"),
            transform: Transform::from_xyz(pos_fr.0, pos_fr.1, Z_INDEX_TEXT),
            ..default()
        })
        .insert(Interactable {
            shape: Shape::Quad(interactable::shapes::Quad {
                width: 200.,
                height: 120.,
            }),
            ..default()
        })
        .insert(Language::FR)
        .id();
    let lang_es = commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("lang/es.png"),
            transform: Transform::from_xyz(pos_es.0, pos_es.1, Z_INDEX_TEXT),
            ..default()
        })
        .insert(Interactable {
            shape: Shape::Quad(interactable::shapes::Quad {
                width: 200.,
                height: 120.,
            }),
            ..default()
        })
        .insert(Language::ES)
        .id();

    let pos_lang_sel = match profile.lang {
        Language::DE => pos_de,
        Language::FR => pos_fr,
        Language::ES => pos_es,
        _ => pos_en,
    };
    let lang_selector = commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(bevy::prelude::shape::Quad::new(Vec2::new(
                    208.0, 128.0,
                ))))
                .into(),
            material: colors.add(ColorMaterial::from(Color::rgb(0.9, 0.9, 0.9))),
            transform: Transform::from_xyz(pos_lang_sel.0, pos_lang_sel.1, Z_INDEX_CELL_BACK),
            ..default()
        })
        .insert(LangSelector)
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
            settings_cell,
            logo_entity,
            lang_en,
            lang_de,
            lang_fr,
            lang_es,
            lang_selector,
        ])
        .insert_bundle(SpatialBundle::from_transform(root_transform))
        .insert(RootComponent);
}
