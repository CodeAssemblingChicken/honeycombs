use super::{
    components::MouseInverted,
    constants::{COLOR_SELECTED, COLOR_UNSELECTED},
};
use crate::{
    assets::LocaleAsset,
    components::{Language, RootComponent},
    constants::{MED_SCALE, RADIUS, Z_INDEX_TEXT},
    functions::rescale_board,
    resources::{LocaleAssets, Profile, TextSettings},
};
use bevy::{
    hierarchy::BuildChildren,
    prelude::{default, AssetServer, Assets, Commands, Res, SpatialBundle, Sprite, Transform},
    sprite::SpriteBundle,
    window::Windows,
};
use interactable::{components::Interactable, shapes::Shape};

pub fn setup(
    mut commands: Commands,
    wnds: Res<Windows>,
    (locale, profile, text_settings): (Res<LocaleAssets>, Res<Profile>, Res<TextSettings>),
    asset_server: Res<AssetServer>,
    locales: Res<Assets<LocaleAsset>>,
) {
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
            sprite: Sprite {
                color: match profile.lang {
                    Language::EN => COLOR_SELECTED,
                    _ => COLOR_UNSELECTED,
                },
                ..default()
            },
            texture: asset_server.load("img/en.png"),
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
            sprite: Sprite {
                color: match profile.lang {
                    Language::DE => COLOR_SELECTED,
                    _ => COLOR_UNSELECTED,
                },
                ..default()
            },
            texture: asset_server.load("img/de.png"),
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
            sprite: Sprite {
                color: match profile.lang {
                    Language::FR => COLOR_SELECTED,
                    _ => COLOR_UNSELECTED,
                },
                ..default()
            },
            texture: asset_server.load("img/fr.png"),
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
            sprite: Sprite {
                color: match profile.lang {
                    Language::ES => COLOR_SELECTED,
                    _ => COLOR_UNSELECTED,
                },
                ..default()
            },
            texture: asset_server.load("img/es.png"),
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

    let pos_lmb = (50.0, 0.);
    let pos_rmb = (280.0, 0.);
    let lmb = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: if profile.mouse_inverted {
                    COLOR_UNSELECTED
                } else {
                    COLOR_SELECTED
                },
                ..default()
            },
            texture: asset_server.load("img/lmb.png"),
            transform: Transform::from_xyz(pos_lmb.0, pos_lmb.1, Z_INDEX_TEXT),
            ..default()
        })
        .insert(Interactable {
            shape: Shape::Quad(interactable::shapes::Quad {
                width: 222.,
                height: 371.,
            }),
            ..default()
        })
        .insert(MouseInverted(false))
        .id();
    let rmb = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: if profile.mouse_inverted {
                    COLOR_SELECTED
                } else {
                    COLOR_UNSELECTED
                },
                ..default()
            },
            texture: asset_server.load("img/rmb.png"),
            transform: Transform::from_xyz(pos_rmb.0, pos_rmb.1, Z_INDEX_TEXT),
            ..default()
        })
        .insert(Interactable {
            shape: Shape::Quad(interactable::shapes::Quad {
                width: 222.,
                height: 371.,
            }),
            ..default()
        })
        .insert(MouseInverted(true))
        .id();

    let mut root_transform = Transform::identity();
    for wnd in wnds.iter() {
        // TODO: Remove hard-coded width/height
        rescale_board(10, 6, 1, wnd.width(), wnd.height(), &mut root_transform);
    }

    commands
        .spawn()
        .push_children(&[logo_entity, lang_en, lang_de, lang_fr, lang_es, lmb, rmb])
        .insert_bundle(SpatialBundle::from_transform(root_transform))
        .insert(RootComponent);
}
