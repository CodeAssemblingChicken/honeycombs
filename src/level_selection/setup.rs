use super::{components::StageCluster, functions::spawn_cluster};
use crate::{
    constants::{RADIUS, UNLOCK_POINTS},
    functions::rescale_board,
    resources::{CellMeshes, GameColors, Profile, TextSettings},
};
use bevy::{
    prelude::{Camera, Commands, Query, Res, Transform, With},
    window::Windows,
};

pub fn setup(
    mut commands: Commands,
    wnds: Res<Windows>,
    cell_meshes: Res<CellMeshes>,
    game_colors: Res<GameColors>,
    profile: Res<Profile>,
    text_settings: Res<TextSettings>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    for (id, (dx, dy)) in [(0, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)]
        .into_iter()
        .enumerate()
    {
        let sc = StageCluster::new(id as u8 + 1, UNLOCK_POINTS[id], 6);
        spawn_cluster(
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
        );
    }

    for wnd in wnds.iter() {
        // TODO: Remove hard-coded width/height
        rescale_board(11, 11, 1, wnd.width(), wnd.height(), &mut camera_query);
    }
}
