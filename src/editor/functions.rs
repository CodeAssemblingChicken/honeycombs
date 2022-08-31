use super::components::{Board, CellUpdateEvent, EditorCell, EmptyCell, NumberCell, UnsetCell};
use crate::{
    components::{Cell, CellType, HintType},
    resources::CellColors,
};
use bevy::{
    prelude::{Commands, Entity, EventWriter, Handle, Query, Visibility},
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
