use super::{
    board::Board,
    components::{CellUpdateEvent, EditorCell, EmptyCell, NumberCell, UnsetCell},
};
use crate::{
    components::{Cell, CellType, HintType},
    constants::{RADIUS, Z_INDEX_CELL_BACK},
    functions::{calc_translation, make_cell_interactable, spawn_cell, spawn_cell_text},
    resources::{CellColors, CellMeshes, TextSettings},
};
use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{default, Commands, Entity, EventWriter, Handle, Query, Transform, Visibility},
    sprite::ColorMaterial,
};

pub fn row_empty(row: &Vec<(Option<CellType>, bool)>) -> bool {
    for (entry, _) in row {
        if entry.is_some() {
            return false;
        }
    }
    true
}

// Spawns a cell with common options. Returns the text_entity for convenience
pub fn spawn_cell_common(
    commands: &mut Commands,
    cell: Entity,
    (cell_meshes, text_settings): (&CellMeshes, &TextSettings),
    colors: (
        Handle<ColorMaterial>,
        Handle<ColorMaterial>,
        Handle<ColorMaterial>,
    ),
    (x, y): (i32, i32),
    (w, h): (f32, f32),
) -> Entity {
    let (tx, ty) = calc_translation(x, y, w, h);
    let mut big_transform = Transform::from_translation(Vec3::new(tx, ty, Z_INDEX_CELL_BACK));
    big_transform.rotate_z(f32::to_radians(90.0));

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

    make_cell_interactable(
        commands,
        cell,
        interactable::click::MouseActions {
            left_just: true,
            left_pressed: true,
            right_just: true,
            right_pressed: true,
            middle_pressed: true,
            ..default()
        },
        RADIUS,
    );

    let cell_component = Cell {
        x,
        y,
        entity: cell,
        outer_hexagon: child1,
        inner_hexagon: child2,
        orig: big_transform,
        hovering: false,
    };

    let text_entity = spawn_cell_text(commands, "0", text_settings);
    commands
        .entity(text_entity)
        .insert(Visibility { is_visible: false });

    commands
        .entity(cell)
        .insert(cell_component)
        .add_child(text_entity);
    text_entity
}

pub fn unset_cell(
    commands: &mut Commands,
    entity: Entity,
    (cell, ec): (&mut Cell, &mut EditorCell),
    color_query: &mut Query<&mut Handle<ColorMaterial>>,
    cell_colors: &CellColors,
    board: &mut Board,
    ev_cell_update: &mut EventWriter<CellUpdateEvent>,
) {
    ec.cell_type = None;
    ec.hidden = false;
    commands.entity(entity).insert(UnsetCell);
    commands
        .entity(ec.text_entity)
        .insert(Visibility { is_visible: false });
    // TODO: Does it really make sense to click here?
    cell.click(
        commands,
        Some(cell_colors.alpha0.clone()),
        cell_colors.alpha0.clone(),
        cell_colors.alpha1.clone(),
        color_query,
    );
    board.cells[cell.y as usize][cell.x as usize].0 = None;
    board.cells[cell.y as usize][cell.x as usize].1 = false;
    ev_cell_update.send(CellUpdateEvent);
}

pub fn set_empty_cell(
    commands: &mut Commands,
    entity: Entity,
    (cell, ec): (&mut Cell, &mut EditorCell),
    color_query: &mut Query<&mut Handle<ColorMaterial>>,
    cell_colors: &CellColors,
    board: &mut Board,
    ev_cell_update: &mut EventWriter<CellUpdateEvent>,
) {
    ec.cell_type = Some(CellType::EmptyCell);
    commands
        .entity(entity)
        .remove::<UnsetCell>()
        .insert(EmptyCell);
    // TODO: Does it really make sense to click here?
    cell.click(
        commands,
        Some(cell_colors.white.clone()),
        cell_colors.blue_light.clone(),
        cell_colors.blue_medium.clone(),
        color_query,
    );
    board.cells[cell.y as usize][cell.x as usize].0 = Some(CellType::EmptyCell);
    ev_cell_update.send(CellUpdateEvent);
}

pub fn set_number_cell(
    commands: &mut Commands,
    entity: Entity,
    (cell, ec): (&mut Cell, &mut EditorCell),
    color_query: &mut Query<&mut Handle<ColorMaterial>>,
    cell_colors: &CellColors,
    board: &mut Board,
    ev_cell_update: &mut EventWriter<CellUpdateEvent>,
) {
    ec.cell_type = Some(CellType::NumberCell(HintType::None));
    let count = 0;
    commands
        .entity(entity)
        .remove::<UnsetCell>()
        .insert(NumberCell {
            count,
            label: ec.text_entity,
            special_hint: false,
        });
    commands
        .entity(ec.text_entity)
        .insert(Visibility { is_visible: true });
    // TODO: Does it really make sense to click here?
    cell.click(
        commands,
        Some(cell_colors.white.clone()),
        cell_colors.gray_light.clone(),
        cell_colors.gray_medium.clone(),
        color_query,
    );
    board.cells[cell.y as usize][cell.x as usize].0 = Some(CellType::NumberCell(HintType::None));
    ev_cell_update.send(CellUpdateEvent);
}
