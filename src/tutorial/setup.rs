use super::components::{ButtonClose, ButtonNext, UiBackground, UiRootNode};
use crate::{
    assets::LocaleAsset,
    constants::Z_INDEX_UI,
    functions::spawn_cell,
    resources::{CellMeshes, GameColors, LocaleAssets, Profile, TextSettings},
};
use bevy::{
    hierarchy::BuildChildren,
    math::{Vec2, Vec3},
    prelude::{default, shape::Quad, Assets, Color, Commands, Mesh, Res, ResMut, Transform},
    sprite::{ColorMaterial, ColorMesh2dBundle},
    text::{Text, Text2dBundle},
    window::Windows,
};
use interactable::{components::Interactable, shapes::Shape};

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
    (cell_meshes, game_colors, locale, profile, text_settings): StandardResources,
    (mut meshes, mut colors, locales): StandardAssets,
    wnds: Res<Windows>,
) {
    // Panel width and height 1920×1080p window
    let (panel_width, panel_height) = (1280., 960.);
    let total_text = format!(
        "{}:",
        locale
            .get_string("total", &locales, &profile)
            .unwrap_or(&"String not found".to_string())
    );

    let total_cell = commands.spawn().id();
    let tf = Transform::from_scale(Vec3::new(1.5, 1.5, 1.0));
    spawn_cell(
        &mut commands,
        total_cell,
        (
            cell_meshes.std_hexagon_back.clone(),
            cell_meshes.std_hexagon_outer.clone(),
            cell_meshes.std_hexagon_inner.clone(),
        ),
        (
            game_colors.alpha2.clone(),
            game_colors.blue_light.clone(),
            game_colors.blue_medium.clone(),
        ),
        tf,
    );

    let mut tf_background = Transform::from_xyz(0., 0., Z_INDEX_UI);
    let mut tf_panel = Transform::from_xyz(0., 0., Z_INDEX_UI + 1.);
    for wnd in wnds.iter() {
        tf_background.scale = Vec3::new(wnd.width(), wnd.height(), 1.0);
        let w = wnd.width() / 1920.;
        let h = wnd.height() / 1080.;
        let s = w.min(h);
        tf_panel.scale = Vec3::new(s, s, 1.0);
    }
    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(Quad::new(Vec2::new(1.0, 1.0))))
                .into(),
            material: colors.add(ColorMaterial::from(Color::rgba(0.0, 0.0, 0.0, 0.3))),
            transform: tf_background,
            ..default()
        })
        .insert(UiBackground);
    commands
        .spawn_bundle(ColorMesh2dBundle {
            mesh: meshes
                .add(Mesh::from(Quad::new(Vec2::new(panel_width, panel_height))))
                .into(),
            material: colors.add(ColorMaterial::from(Color::rgba(0.8, 0.8, 0.8, 0.9))),
            transform: tf_panel,
            ..default()
        })
        .insert(UiRootNode)
        .with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    locale
                        .get_string("complete", &locales, &profile)
                        .unwrap_or(&"String not found".to_string())
                        .clone(),
                    text_settings.style_menu_dark.clone(),
                )
                .with_alignment(text_settings.alignment),
                transform: Transform::from_xyz(0., 400., 1.),
                ..default()
            });
            parent.spawn_bundle(ColorMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(Quad::new(Vec2::new(600., 580.))))
                    .into(),
                material: colors.add(ColorMaterial::from(Color::rgba(0.7, 0.7, 0.7, 0.92))),
                transform: Transform::from_xyz(-310., 50., 0.9),
                ..default()
            });
            parent
                .spawn_bundle(ColorMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(Quad::new(Vec2::new(600., 580.))))
                        .into(),
                    material: colors.add(ColorMaterial::from(Color::rgba(0.7, 0.7, 0.7, 0.92))),
                    transform: Transform::from_xyz(310., 50., 0.9),
                    ..default()
                })
                .add_child(total_cell)
                .with_children(|parent| {
                    parent.spawn_bundle(Text2dBundle {
                        text: Text::from_section(total_text, text_settings.style_menu_dark.clone())
                            .with_alignment(text_settings.alignment),
                        transform: Transform::from_xyz(0., 200., 10.),
                        ..default()
                    });
                    parent.spawn_bundle(Text2dBundle {
                        text: Text::from_section(
                            format!("×{}", profile.get_points()),
                            text_settings.style_menu_dark.clone(),
                        )
                        .with_alignment(text_settings.alignment),
                        transform: Transform::from_xyz(0., -210., 10.),
                        ..default()
                    });
                });
            parent
                .spawn_bundle(ColorMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(Quad::new(Vec2::new(240., 190.))))
                        .into(),
                    material: colors.add(ColorMaterial::from(Color::rgba(0.7, 0.7, 0.7, 0.92))),
                    transform: Transform::from_xyz(0., -355., 0.9),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(Text2dBundle {
                        text: Text::from_section(
                            locale
                                .get_string("next", &locales, &profile)
                                .unwrap_or(&"String not found".to_string()),
                            text_settings.style_menu_dark.clone(),
                        )
                        .with_alignment(text_settings.alignment),
                        transform: Transform::from_xyz(0., -10., 10.)
                            .with_scale(Vec3::new(0.75, 0.75, 1.)),
                        ..default()
                    });
                })
                .insert(ButtonNext)
                .insert(Interactable {
                    shape: Shape::Quad(interactable::shapes::Quad {
                        width: 240.,
                        height: 190.,
                    }),
                    ..default()
                });

            parent
                .spawn_bundle(ColorMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(Quad::new(Vec2::new(240., 190.))))
                        .into(),
                    material: colors.add(ColorMaterial::from(Color::rgba(0.7, 0.7, 0.7, 0.92))),
                    transform: Transform::from_xyz(260., -355., 0.9),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(Text2dBundle {
                        text: Text::from_section(
                            locale
                                .get_string("menu", &locales, &profile)
                                .unwrap_or(&"String not found".to_string()),
                            text_settings.style_menu_dark.clone(),
                        )
                        .with_alignment(text_settings.alignment),
                        transform: Transform::from_xyz(0., -10., 10.)
                            .with_scale(Vec3::new(0.75, 0.75, 1.)),
                        ..default()
                    });
                })
                .insert(ButtonClose)
                .insert(Interactable {
                    shape: Shape::Quad(interactable::shapes::Quad {
                        width: 240.,
                        height: 190.,
                    }),
                    ..default()
                });
        });
}
