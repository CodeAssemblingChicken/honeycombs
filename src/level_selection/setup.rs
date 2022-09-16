use super::{
    components::{ButtonReturn, ButtonTutorial, StageCluster},
    functions::spawn_cluster,
};
use crate::{
    assets::LocaleAsset,
    bundles::MenuButtonBundle,
    components::RootComponent,
    constants::{RADIUS, UNLOCK_POINTS, Z_INDEX_TEXT},
    functions::{rescale_board, spawn_cell},
    resources::{CellMeshes, GameColors, LocaleAssets, Profile, TextSettings},
};
use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{default, Assets, Commands, Mesh, Res, ResMut, SpatialBundle, Transform},
    text::{Text, Text2dBundle},
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
    (mut meshes, locales): (ResMut<Assets<Mesh>>, Res<Assets<LocaleAsset>>),
) {
    let mut clusters = Vec::new();
    for (id, (dx, dy)) in [(0, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)]
        .into_iter()
        .enumerate()
    {
        let sc = StageCluster::new(id as u8, UNLOCK_POINTS[id], 6);
        clusters.push(spawn_cluster(
            &mut commands,
            &cell_meshes,
            &game_colors,
            &profile,
            &text_settings,
            sc,
            (
                dx as f32 * RADIUS * 1.56 * 4.,
                dy as f32 * RADIUS * -1.8 * 4.
                    + match dx == 0 {
                        true => 0.,
                        false => RADIUS * 0.9 * 4.,
                    },
            ),
        ));
    }

    let center_cell = commands.spawn().id();
    spawn_cell(
        &mut commands,
        center_cell,
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
        Transform::from_scale(Vec3::new(2., 2., 1.)),
    );
    commands.entity(center_cell).with_children(|parent| {
        parent.spawn_bundle(Text2dBundle {
            text: Text::from_section(
                format!("{} /\n{}", profile.get_points(), 447),
                text_settings.style_cell.clone(),
            )
            .with_alignment(text_settings.alignment),
            transform: Transform::from_xyz(0., 0., Z_INDEX_TEXT)
                .with_scale(Vec3::new(0.7, 0.7, 1.0)),
            ..default()
        });
    });

    let bt_tutorial = commands
        .spawn_bundle(MenuButtonBundle::new(
            Transform::from_xyz(-6.25 * RADIUS, -8. * RADIUS, 0.9),
            (240., 150.),
            game_colors.menu_button.clone(),
            &mut meshes,
        ))
        .with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    locale
                        .get_string("tutorial", &locales, &profile)
                        .unwrap_or(&"String not found".to_string()),
                    text_settings.style_menu_dark.clone(),
                )
                .with_alignment(text_settings.alignment),
                transform: Transform::from_xyz(0., -10., 10.).with_scale(Vec3::new(0.75, 0.75, 1.)),
                ..default()
            });
        })
        .insert(ButtonTutorial)
        .id();
    let bt_return = commands
        .spawn_bundle(MenuButtonBundle::new(
            Transform::from_xyz(6.25 * RADIUS, -8. * RADIUS, 0.9),
            (240., 150.),
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
        .id();

    let mut root_transform = Transform::identity();
    for wnd in wnds.iter() {
        // TODO: Remove hard-coded width/height
        rescale_board(11, 11, 1, wnd.width(), wnd.height(), &mut root_transform);
    }
    commands
        .spawn()
        .push_children(&clusters)
        .push_children(&[center_cell, bt_tutorial, bt_return])
        .insert_bundle(SpatialBundle::from_transform(root_transform))
        .insert(RootComponent);
}
