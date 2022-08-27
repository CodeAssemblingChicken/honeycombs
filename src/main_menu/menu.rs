use super::components::{spawn_cluster, StageCluster};
use crate::{
    constants::RADIUS,
    functions::rescale_board,
    resources::{CellColors, CellMeshes, TextSettings},
};
use bevy::{
    prelude::{Camera, Commands, Query, Res, Transform, With},
    window::Windows,
};

pub fn setup(
    mut commands: Commands,
    wnds: Res<Windows>,
    cell_meshes: Res<CellMeshes>,
    cell_colors: Res<CellColors>,
    text_settings: Res<TextSettings>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    for (id, (dx, dy)) in [(0, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)]
        .into_iter()
        .enumerate()
    {
        let sc = StageCluster::new(id as u8 + 1, 0, 6);
        spawn_cluster(
            &mut commands,
            &cell_meshes,
            &cell_colors,
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
        );
    }
    for w in wnds.iter() {
        // TODO: Remove hard-coded width/height
        rescale_board(11, 11, 1, w.width(), w.height(), &mut camera_query);
    }
}
