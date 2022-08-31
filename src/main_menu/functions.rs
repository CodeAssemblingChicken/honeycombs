use super::components::{LevelSelectionCell, OptionCell, StageCluster};
use crate::{
    components::Cell,
    constants::{MED_SCALE, RADIUS, Z_INDEX_CELL_BACK},
    functions::{calc_translation, make_cell_interactable, spawn_cell, spawn_cell_text},
    resources::{CellColors, CellMeshes, TextSettings},
    states::AppState,
};
use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{Commands, Transform},
};

pub fn spawn_cluster(
    commands: &mut Commands,
    cell_meshes: &CellMeshes,
    cell_colors: &CellColors,
    text_settings: &TextSettings,
    stage_cluster: StageCluster,
    (x, y): (f32, f32),
) {
    let mut big_transform = Transform::from_translation(Vec3::new(x, y, Z_INDEX_CELL_BACK));
    big_transform.rotate_z(f32::to_radians(90.0));

    for (id, (dx, dy)) in [(0, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)]
        .into_iter()
        .take(stage_cluster.num_levels as usize)
        .enumerate()
    {
        let (tx, ty) = calc_translation(dx, dy, 0., 0.);
        let mut big_transform =
            Transform::from_translation(Vec3::new(x + tx, y + ty, Z_INDEX_CELL_BACK));
        big_transform.rotate_z(f32::to_radians(90.0));
        spawn_level_selection_cell(
            commands,
            cell_meshes,
            cell_colors,
            text_settings,
            big_transform,
            id as u8 + 1,
            stage_cluster.stage_no,
        );
    }
}

fn spawn_level_selection_cell(
    commands: &mut Commands,
    cell_meshes: &CellMeshes,
    cell_colors: &CellColors,
    text_settings: &TextSettings,
    big_transform: Transform,
    level_id: u8,
    stage_id: u8,
) {
    let cell = commands.spawn().id();
    let (child1, child2) = spawn_cell(
        commands,
        cell,
        (
            cell_meshes.std_hexagon_back.clone(),
            cell_meshes.std_hexagon_outer.clone(),
            cell_meshes.std_hexagon_inner.clone(),
        ),
        (
            cell_colors.white.clone(),
            cell_colors.blue_medium.clone(),
            cell_colors.blue_light.clone(),
        ),
        big_transform,
    );

    let text_entity = spawn_cell_text(
        commands,
        &format!("{}–{}", stage_id, level_id),
        text_settings,
    );
    commands.entity(cell).add_child(text_entity);

    make_cell_interactable(commands, cell, (true, false, false), RADIUS);

    let cell_component = Cell {
        x: stage_id as i32,
        y: level_id as i32,
        entity: cell,
        outer_hexagon: child1,
        inner_hexagon: child2,
        orig: big_transform,
        hovering: false,
    };
    commands
        .entity(cell)
        .insert(cell_component)
        .insert(LevelSelectionCell {
            stage: stage_id,
            level: level_id,
        });
}

pub fn spawn_option_cell(
    commands: &mut Commands,
    cell_meshes: &CellMeshes,
    cell_colors: &CellColors,
    text_settings: &TextSettings,
    big_transform: Transform,
    app_state: AppState,
    text: &str,
) {
    let cell = commands.spawn().id();
    let (child1, child2) = spawn_cell(
        commands,
        cell,
        (
            cell_meshes.med_hexagon_back.clone(),
            cell_meshes.med_hexagon_outer.clone(),
            cell_meshes.med_hexagon_inner.clone(),
        ),
        (
            cell_colors.white.clone(),
            cell_colors.gray_medium.clone(),
            cell_colors.gray_light.clone(),
        ),
        big_transform,
    );

    let text_entity = spawn_cell_text(commands, text, text_settings);
    commands.entity(cell).add_child(text_entity);

    make_cell_interactable(commands, cell, (true, false, false), RADIUS * MED_SCALE);

    let cell_component = Cell {
        x: -1,
        y: -1,
        entity: cell,
        outer_hexagon: child1,
        inner_hexagon: child2,
        orig: big_transform,
        hovering: false,
    };
    commands
        .entity(cell)
        .insert(cell_component)
        .insert(OptionCell { app_state });
}
