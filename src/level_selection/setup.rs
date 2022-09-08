use super::{components::StageCluster, functions::spawn_cluster};
use crate::{
    components::RootComponent,
    constants::{RADIUS, UNLOCK_POINTS, Z_INDEX_TEXT},
    functions::{rescale_board, spawn_cell},
    resources::{CellMeshes, GameColors, Profile, TextSettings},
};
use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{default, Commands, Res, SpatialBundle, Transform},
    text::{Text, Text2dBundle},
    window::Windows,
};

pub fn setup(
    mut commands: Commands,
    wnds: Res<Windows>,
    cell_meshes: Res<CellMeshes>,
    game_colors: Res<GameColors>,
    profile: Res<Profile>,
    text_settings: Res<TextSettings>,
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
    let mut root_transform = Transform::identity();
    for wnd in wnds.iter() {
        // TODO: Remove hard-coded width/height
        rescale_board(11, 11, 1, wnd.width(), wnd.height(), &mut root_transform);
    }
    commands
        .spawn()
        .push_children(&clusters)
        .add_child(center_cell)
        .insert_bundle(SpatialBundle::from_transform(root_transform))
        .insert(RootComponent);
}
