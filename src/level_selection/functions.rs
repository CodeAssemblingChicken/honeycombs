use super::components::{LevelSelectionCell, StageCluster};
use crate::{
    components::Cell,
    constants::{RADIUS, Z_INDEX_CELL_BACK},
    functions::{calc_translation, make_cell_interactable, spawn_cell, spawn_cell_text},
    resources::{CellMeshes, GameColors, Profile, TextSettings},
};
use bevy::{
    hierarchy::BuildChildren,
    prelude::{Commands, Entity, SpatialBundle, Transform},
};

pub fn spawn_cluster(
    commands: &mut Commands,
    cell_meshes: &CellMeshes,
    game_colors: &GameColors,
    profile: &Profile,
    text_settings: &TextSettings,
    stage_cluster: StageCluster,
    (x, y): (f32, f32),
) -> Entity {
    let unlocked = profile.get_points() >= stage_cluster.unlock_required;

    let mut big_transform = Transform::from_xyz(x, y, Z_INDEX_CELL_BACK);
    big_transform.rotate_z(f32::to_radians(90.0));

    let mut ls_cells = Vec::new();
    for (id, (dx, dy)) in [(0, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)]
        .into_iter()
        .take(stage_cluster.num_levels as usize)
        .enumerate()
    {
        let (tx, ty) = calc_translation(dx, dy, 0., 0.);
        let mut big_transform = Transform::from_xyz(x + tx, y + ty, Z_INDEX_CELL_BACK);
        big_transform.rotate_z(f32::to_radians(90.0));
        ls_cells.push(spawn_level_selection_cell(
            commands,
            cell_meshes,
            game_colors,
            profile,
            text_settings,
            big_transform,
            (id as u8, stage_cluster.stage_no, unlocked),
        ));
    }
    let (tx, ty) = calc_translation(0, 0, 0., 0.);
    let mut big_transform = Transform::from_xyz(x + tx, y + ty, Z_INDEX_CELL_BACK);
    big_transform.rotate_z(f32::to_radians(90.0));
    let cluster_cell = spawn_cluster_cell(
        commands,
        cell_meshes,
        game_colors,
        text_settings,
        big_transform,
        (stage_cluster.unlock_required, unlocked),
    );
    commands
        .spawn()
        .push_children(&ls_cells)
        .push_children(&[cluster_cell])
        .insert_bundle(SpatialBundle::default())
        .id()
}

fn spawn_level_selection_cell(
    commands: &mut Commands,
    cell_meshes: &CellMeshes,
    game_colors: &GameColors,
    profile: &Profile,
    text_settings: &TextSettings,
    big_transform: Transform,
    (level_id, stage_id, unlocked): (u8, u8, bool),
) -> Entity {
    let cell = commands.spawn().id();

    let colors = if unlocked {
        make_cell_interactable(commands, cell, RADIUS);
        if profile.level_points[stage_id as usize][level_id as usize].is_some() {
            (
                game_colors.white.clone(),
                game_colors.blue_medium.clone(),
                game_colors.blue_light.clone(),
            )
        } else {
            (
                game_colors.white.clone(),
                game_colors.yellow_medium.clone(),
                game_colors.yellow_light.clone(),
            )
        }
    } else {
        (
            game_colors.white.clone(),
            game_colors.gray_medium.clone(),
            game_colors.gray_light.clone(),
        )
    };

    let (child1, child2) = spawn_cell(
        commands,
        cell,
        (
            cell_meshes.std_hexagon_back.clone(),
            cell_meshes.std_hexagon_outer.clone(),
            cell_meshes.std_hexagon_inner.clone(),
        ),
        colors,
        big_transform,
    );

    let text_entity = spawn_cell_text(
        commands,
        &format!("{}???{}", stage_id + 1, level_id + 1),
        text_settings.style_cell.clone(),
        text_settings.alignment,
    );
    commands.entity(cell).add_child(text_entity);

    let cell_component = Cell {
        x: stage_id as i32,
        y: level_id as i32,
        entity: cell,
        outer_hexagon: child1,
        inner_hexagon: child2,
        orig: big_transform,
    };
    commands
        .entity(cell)
        .insert(cell_component)
        .insert(LevelSelectionCell {
            stage: stage_id,
            level: level_id,
        });
    cell
}

fn spawn_cluster_cell(
    commands: &mut Commands,
    cell_meshes: &CellMeshes,
    game_colors: &GameColors,
    text_settings: &TextSettings,
    big_transform: Transform,
    (unlock_required, unlocked): (u16, bool),
) -> Entity {
    let cell = commands.spawn().id();
    let colors = if unlocked {
        (
            game_colors.alpha0.clone(),
            game_colors.blue_light.clone(),
            game_colors.blue_medium.clone(),
        )
    } else {
        (
            game_colors.alpha0.clone(),
            game_colors.gray_light.clone(),
            game_colors.gray_medium.clone(),
        )
    };
    spawn_cell(
        commands,
        cell,
        (
            cell_meshes.std_hexagon_back.clone(),
            cell_meshes.std_hexagon_outer.clone(),
            cell_meshes.std_hexagon_inner.clone(),
        ),
        colors,
        big_transform,
    );

    let text_entity = spawn_cell_text(
        commands,
        &format!("{}", unlock_required),
        text_settings.style_cell.clone(),
        text_settings.alignment,
    );
    commands.entity(cell).add_child(text_entity);
    cell
}
