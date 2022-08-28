use crate::{
    components::Cell,
    constants::Z_INDEX_CELL_BACK,
    functions::{calc_translation, make_cell_interactable, spawn_cell, spawn_cell_text},
    resources::{CellColors, CellMeshes, LevelFile, TextSettings},
    states::AppState,
};
use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{Commands, Component, Handle, Query, ResMut, State, Transform},
    sprite::ColorMaterial,
};

#[derive(Component)]
pub struct LevelSelectionCell {
    pub stage: u8,
    pub level: u8,
}

impl LevelSelectionCell {
    pub fn hover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
    ) {
        cell.hover(
            commands,
            None,
            cell_colors.blue_medium.clone(),
            cell_colors.blue_dark.clone(),
            color_query,
        );
    }

    pub fn unhover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
    ) {
        cell.unhover(
            commands,
            None,
            cell_colors.blue_light.clone(),
            cell_colors.blue_medium.clone(),
            color_query,
        );
    }

    pub fn click(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
        app_state: &mut ResMut<State<AppState>>,
        level_file: &mut ResMut<LevelFile>,
    ) {
        if cell.hovering {
            cell.hovering = false;
        }
        cell.click(
            commands,
            None,
            cell_colors.blue_light.clone(),
            cell_colors.blue_medium.clone(),
            color_query,
        );
        level_file.filename = Some(format!("assets/levels/{}/{}.lvl", self.stage, self.level));
        app_state.set(AppState::Level).unwrap();
    }
}

pub struct StageCluster {
    pub stage_no: u8,
    pub unlock_required: u32,
    pub num_levels: u8,
}

impl StageCluster {
    pub fn new(stage_no: u8, unlock_required: u32, num_levels: u8) -> Self {
        assert!(num_levels > 0 && num_levels <= 6);
        Self {
            stage_no,
            unlock_required,
            num_levels,
        }
    }
}

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
        cell_meshes,
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

    make_cell_interactable(commands, cell, (true, false));

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
