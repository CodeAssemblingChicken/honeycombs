use bevy::{
    prelude::{Camera, Commands, Query, Res, Transform, With},
    window::Windows,
};

use crate::resources::{CellColors, CellMeshes, TextSettings};

pub fn setup(
    mut commands: Commands,
    wnds: Res<Windows>,
    cell_meshes: Res<CellMeshes>,
    cell_colors: Res<CellColors>,
    text_settings: Res<TextSettings>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
}
