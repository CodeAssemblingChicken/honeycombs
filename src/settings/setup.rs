use super::{
    components::{ButtonReturn, ButtonWindowMode, MouseInverted, SettingsButton, TextWindowMode},
    constants::{COLOR_SELECTED, COLOR_UNSELECTED},
    functions::window_mode_text,
};
use crate::{
    assets::LocaleAsset,
    bundles::MenuButtonBundle,
    components::{Language, RootComponent},
    constants::{MED_SCALE, RADIUS, Z_INDEX_TEXT},
    functions::rescale_board,
    resources::{GameColors, LocaleAssets, Profile, TextSettings},
};
use bevy::{
    hierarchy::BuildChildren,
    math::{Vec2, Vec3},
    prelude::{
        default, shape::Quad, AssetServer, Assets, Color, ColorMaterial, ColorMesh2dBundle,
        Commands, Mesh, Res, ResMut, SpatialBundle, Sprite, Transform,
    },
    sprite::SpriteBundle,
    text::{Text, Text2dBundle},
    window::Windows,
};
use interactable::{components::Interactable, shapes::Shape};

type StandardAssets<'a> = (
    ResMut<'a, Assets<Mesh>>,
    ResMut<'a, Assets<ColorMaterial>>,
    Res<'a, Assets<LocaleAsset>>,
);
pub fn setup(
    mut commands: Commands,
    wnds: Res<Windows>,
    (game_colors, locale, profile, text_settings): (
        Res<GameColors>,
        Res<LocaleAssets>,
        Res<Profile>,
        Res<TextSettings>,
    ),
    asset_server: Res<AssetServer>,
    (mut meshes, mut colors, locales): StandardAssets,
) {
    let logo_entity = commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("branding/logo.png"),
            transform: Transform::from_xyz(0., 2. * RADIUS * MED_SCALE, Z_INDEX_TEXT),
            ..default()
        })
        .id();

    let language_panel = commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(Quad::new(Vec2::new(540., 500.))))
                .into(),
            material: colors.add(ColorMaterial::from(Color::rgba(0.7, 0.7, 0.7, 0.92))),
            transform: Transform::from_xyz(4.0 * RADIUS, -RADIUS, 0.9),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    locale
                        .get_string("language", &locales, &profile)
                        .unwrap_or(&"String not found".to_string()),
                    text_settings.style_menu_dark.clone(),
                )
                .with_alignment(text_settings.alignment),
                transform: Transform::from_xyz(0., 1.5 * RADIUS, Z_INDEX_TEXT),
                ..default()
            });
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: match profile.lang {
                            Language::EN => COLOR_SELECTED,
                            _ => COLOR_UNSELECTED,
                        },
                        ..default()
                    },
                    texture: asset_server.load("img/en.png"),
                    transform: Transform::from_xyz(-140., 0.3 * RADIUS, Z_INDEX_TEXT),
                    ..default()
                })
                .insert(Interactable {
                    shape: Shape::Quad(interactable::shapes::Quad {
                        width: 200.,
                        height: 120.,
                    }),
                    ..default()
                })
                .insert(Language::EN);
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: match profile.lang {
                            Language::DE => COLOR_SELECTED,
                            _ => COLOR_UNSELECTED,
                        },
                        ..default()
                    },
                    texture: asset_server.load("img/de.png"),
                    transform: Transform::from_xyz(140., 0.3 * RADIUS, Z_INDEX_TEXT),
                    ..default()
                })
                .insert(Interactable {
                    shape: Shape::Quad(interactable::shapes::Quad {
                        width: 200.,
                        height: 120.,
                    }),
                    ..default()
                })
                .insert(Language::DE);
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: match profile.lang {
                            Language::FR => COLOR_SELECTED,
                            _ => COLOR_UNSELECTED,
                        },
                        ..default()
                    },
                    texture: asset_server.load("img/fr.png"),
                    transform: Transform::from_xyz(-140., -1.3 * RADIUS, Z_INDEX_TEXT),
                    ..default()
                })
                .insert(Interactable {
                    shape: Shape::Quad(interactable::shapes::Quad {
                        width: 200.,
                        height: 120.,
                    }),
                    ..default()
                })
                .insert(Language::FR);
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: match profile.lang {
                            Language::ES => COLOR_SELECTED,
                            _ => COLOR_UNSELECTED,
                        },
                        ..default()
                    },
                    texture: asset_server.load("img/es.png"),
                    transform: Transform::from_xyz(140., -1.3 * RADIUS, Z_INDEX_TEXT),
                    ..default()
                })
                .insert(Interactable {
                    shape: Shape::Quad(interactable::shapes::Quad {
                        width: 200.,
                        height: 120.,
                    }),
                    ..default()
                })
                .insert(Language::ES);
        })
        .id();

    let mouse_panel = commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(Quad::new(Vec2::new(540., 500.))))
                .into(),
            material: colors.add(ColorMaterial::from(Color::rgba(0.7, 0.7, 0.7, 0.92))),
            transform: Transform::from_xyz(-4.0 * RADIUS, -RADIUS, 0.9),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    locale
                        .get_string("mouse-buttons", &locales, &profile)
                        .unwrap_or(&"String not found".to_string()),
                    text_settings.style_menu_dark.clone(),
                )
                .with_alignment(text_settings.alignment),
                transform: Transform::from_xyz(0., 1.5 * RADIUS, Z_INDEX_TEXT),
                ..default()
            });
            parent
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
                    transform: Transform::from_xyz(-1.2 * RADIUS, -0.6 * RADIUS, Z_INDEX_TEXT),
                    ..default()
                })
                .insert(Interactable {
                    shape: Shape::Quad(interactable::shapes::Quad {
                        width: 167.,
                        height: 278.,
                    }),
                    ..default()
                })
                .insert(MouseInverted(false));
            parent
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
                    transform: Transform::from_xyz(1.2 * RADIUS, -0.6 * RADIUS, Z_INDEX_TEXT),
                    ..default()
                })
                .insert(Interactable {
                    shape: Shape::Quad(interactable::shapes::Quad {
                        width: 167.,
                        height: 278.,
                    }),
                    ..default()
                })
                .insert(MouseInverted(true));
        })
        .id();

    let bt_window_mode = commands
        .spawn_bundle(MenuButtonBundle::new(
            Transform::from_xyz(-400., -4.5 * RADIUS, 0.9),
            (270., 170.),
            game_colors.menu_button.clone(),
            &mut meshes,
        ))
        .with_children(|parent| {
            parent
                .spawn_bundle(Text2dBundle {
                    text: window_mode_text(&locale, &locales, &profile, &text_settings),
                    transform: Transform::from_xyz(0., -10., 10.)
                        .with_scale(Vec3::new(0.75, 0.75, 1.)),
                    ..default()
                })
                .insert(TextWindowMode);
        })
        .insert(ButtonWindowMode)
        .insert(SettingsButton)
        .id();

    let bt_return = commands
        .spawn_bundle(MenuButtonBundle::new(
            Transform::from_xyz(400., -4.5 * RADIUS, 0.9),
            (270., 170.),
            game_colors.menu_button.clone(),
            &mut meshes,
        ))
        .with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    locale
                        .get_string("return", &locales, &profile)
                        .unwrap_or(&"String not found".to_string()),
                    text_settings.style_menu_dark.clone(),
                )
                .with_alignment(text_settings.alignment),
                transform: Transform::from_xyz(0., -10., 10.).with_scale(Vec3::new(0.75, 0.75, 1.)),
                ..default()
            });
        })
        .insert(ButtonReturn)
        .insert(SettingsButton)
        .id();

    let mut root_transform = Transform::identity();
    for wnd in wnds.iter() {
        // TODO: Remove hard-coded width/height
        rescale_board(10, 6, 1, wnd.width(), wnd.height(), &mut root_transform);
    }

    commands
        .spawn()
        .push_children(&[
            logo_entity,
            language_panel,
            mouse_panel,
            bt_window_mode,
            bt_return,
        ])
        .insert_bundle(SpatialBundle::from_transform(root_transform))
        .insert(RootComponent);
}
