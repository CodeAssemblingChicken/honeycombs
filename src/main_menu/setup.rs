use super::{
    components::StageCluster,
    functions::{spawn_cluster, spawn_option_cell},
};
use crate::{
    constants::{MED_SCALE, RADIUS, Z_INDEX_CELL_BACK},
    functions::rescale_board,
    resources::{CellColors, CellMeshes, TextSettings},
    states::AppState,
};
use bevy::{
    math::Vec3,
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
    let mut big_transform = Transform::from_translation(Vec3::new(
        -1.56 * 4. * RADIUS,
        -10. * RADIUS + RADIUS * MED_SCALE,
        Z_INDEX_CELL_BACK,
    ));
    big_transform.rotate_z(f32::to_radians(90.0));
    spawn_option_cell(
        &mut commands,
        &cell_meshes,
        &cell_colors,
        &text_settings,
        big_transform,
        AppState::Editor,
        "Editor",
    );
    big_transform.translation = Vec3::new(
        1.56 * 4. * RADIUS,
        -10. * RADIUS + RADIUS * MED_SCALE,
        Z_INDEX_CELL_BACK,
    );
    spawn_option_cell(
        &mut commands,
        &cell_meshes,
        &cell_colors,
        &text_settings,
        big_transform,
        AppState::Settings,
        "Options",
    );
    for wnd in wnds.iter() {
        // TODO: Remove hard-coded width/height
        rescale_board(11, 11, 1, wnd.width(), wnd.height(), &mut camera_query);
    }
}
