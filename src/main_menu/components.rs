use crate::{
    components::{Cell, CellInner, CellOuter},
    constants::{RADIUS, Z_INDEX_CELL_BACK, Z_INDEX_CELL_INNER, Z_INDEX_CELL_OUTER},
    functions::spawn_cell_text,
    level::resources::LevelFile,
    resources::{CellColors, CellMeshes, TextSettings},
    states::AppState,
};
use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{default, Commands, Component, Handle, Query, ResMut, State, Transform},
    sprite::{ColorMaterial, ColorMesh2dBundle},
};
use interactable::{
    click::Clickable,
    hover::Hoverable,
    shapes::{Hexagon, Shape},
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
    let medium_transform = Transform::from_translation(Vec3::new(0.0, 0.0, Z_INDEX_CELL_OUTER));
    let small_transform = Transform::from_translation(Vec3::new(0.0, 0.0, Z_INDEX_CELL_INNER));

    for (id, (dx, dy)) in [(0, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0)]
        .into_iter()
        .take(stage_cluster.num_levels as usize)
        .enumerate()
    {
        let mut big_transform = Transform::from_translation(Vec3::new(
            x + dx as f32 * RADIUS * 1.56,
            y + dy as f32 * RADIUS * -1.8
                + match dx == 0 {
                    true => 0.,
                    false => RADIUS * 0.9,
                },
            Z_INDEX_CELL_BACK,
        ));
        big_transform.rotate_z(f32::to_radians(90.0));
        spawn_level_selection_cell(
            commands,
            cell_meshes,
            cell_colors,
            text_settings,
            big_transform,
            medium_transform,
            small_transform,
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
    medium_transform: Transform,
    small_transform: Transform,
    level_id: u8,
    stage_id: u8,
) {
    let b1 = ColorMesh2dBundle {
        mesh: cell_meshes.medium_hexagon.clone().into(),
        material: cell_colors.blue_medium.clone(),
        transform: medium_transform,
        ..default()
    };
    let b2 = ColorMesh2dBundle {
        mesh: cell_meshes.small_hexagon.clone().into(),
        material: cell_colors.blue_light.clone(),
        transform: small_transform,
        ..default()
    };

    // do the same for the child
    let child1 = commands.spawn_bundle(b1).insert(CellOuter).id();
    let child2 = commands.spawn_bundle(b2).insert(CellInner).id();

    let cell = commands
        .spawn()
        .insert_bundle(ColorMesh2dBundle {
            mesh: cell_meshes.big_hexagon.clone().into(),
            material: cell_colors.white.clone(),
            transform: big_transform,
            ..default()
        })
        .id();
    commands.entity(cell).push_children(&[child1, child2]);
    let text_entity = spawn_cell_text(
        commands,
        &format!("{}–{}", stage_id, level_id),
        text_settings,
    );
    commands.entity(cell).add_child(text_entity);

    commands
        .entity(cell)
        .insert(Hoverable {
            ignore_scale: true,
            shape: Shape::Hexagon(Hexagon {
                radius: RADIUS,
                point_up: false,
            }),
            ..default()
        })
        .insert(Clickable {
            ignore_scale: true,
            shape: Shape::Hexagon(Hexagon {
                radius: RADIUS,
                point_up: false,
            }),
            left_released: true,
            ..default()
        });

    let cell_component = Cell {
        x: stage_id as usize,
        y: level_id as usize,
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
