use super::{components::StageCluster, functions::spawn_cluster};
use crate::{
    components::RootComponent,
    constants::{RADIUS, UNLOCK_POINTS},
    functions::rescale_board,
    resources::{CellMeshes, GameColors, Profile, TextSettings},
};
use bevy::{
    hierarchy::BuildChildren,
    prelude::{Commands, Res, SpatialBundle, Transform},
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
    let mut root_transform = Transform::identity();
    for wnd in wnds.iter() {
        // TODO: Remove hard-coded width/height
        rescale_board(11, 11, 1, wnd.width(), wnd.height(), &mut root_transform);
    }
    commands
        .spawn()
        .push_children(&clusters)
        .insert_bundle(SpatialBundle::from_transform(root_transform))
        .insert(RootComponent);
}
