use super::functions::row_empty;
use crate::{
    components::{Cell, CellType},
    resources::CellColors,
};
use bevy::{
    prelude::{Commands, Component, Entity, EventWriter, Handle, Query},
    sprite::ColorMaterial,
};

pub struct Board {
    pub cells: Vec<Vec<(Option<CellType>, bool)>>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![vec![(None, false); width]; height],
            width,
            height,
        }
    }
    pub fn trim(&self) -> Vec<Vec<(Option<CellType>, bool)>> {
        let mut cells = self.cells.clone();
        // Check rows
        loop {
            if cells.len() < 3 || !row_empty(&cells[0]) || !row_empty(&cells[1]) {
                break;
            }
            cells.remove(0);
            cells.remove(0);
        }
        loop {
            if cells.len() < 2 || !row_empty(&cells[cells.len() - 1]) {
                break;
            }
            cells.remove(cells.len() - 1);
        }
        // Check columns
        loop {
            if cells[0].len() < 3
                || !row_empty(&(&cells).iter().map(|row| row[0]).collect())
                || !row_empty(&(&cells).iter().map(|row| row[1]).collect())
            {
                break;
            }
            for row in &mut cells {
                *row = row.clone().into_iter().skip(2).collect();
            }
        }
        loop {
            if cells[0].len() < 2
                || !row_empty(&(&cells).iter().map(|row| row[row.len() - 1]).collect())
            {
                break;
            }
            for row in &mut cells {
                *row = row.clone().into_iter().take(row.len() - 1).collect();
            }
        }
        cells
    }
}

#[derive(Component)]
pub struct EditorCell {
    pub hidden: bool,
    pub cell_type: Option<CellType>,
    pub text_entity: Entity,
}

impl EditorCell {
    pub fn hover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
    ) {
        let (light, dark) = if self.hidden {
            (
                cell_colors.yellow_medium.clone(),
                cell_colors.yellow_dark.clone(),
            )
        } else {
            match self.cell_type {
                Some(CellType::NumberCell(_)) => (
                    cell_colors.gray_medium.clone(),
                    cell_colors.gray_dark.clone(),
                ),
                Some(CellType::EmptyCell) => (
                    cell_colors.blue_medium.clone(),
                    cell_colors.blue_dark.clone(),
                ),
                None => (cell_colors.alpha0.clone(), cell_colors.alpha2.clone()),
            }
        };
        cell.hover(commands, None, light, dark, color_query);
    }

    pub fn unhover(
        &self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
    ) {
        let (light, dark) = if let Some(ct) = self.cell_type {
            if self.hidden {
                (
                    cell_colors.yellow_light.clone(),
                    cell_colors.yellow_medium.clone(),
                )
            } else {
                match ct {
                    CellType::NumberCell(_) => (
                        cell_colors.gray_light.clone(),
                        cell_colors.gray_medium.clone(),
                    ),
                    CellType::EmptyCell => (
                        cell_colors.blue_light.clone(),
                        cell_colors.blue_medium.clone(),
                    ),
                }
            }
        } else {
            (cell_colors.alpha0.clone(), cell_colors.alpha1.clone())
        };
        cell.unhover(commands, None, light, dark, color_query);
    }

    pub fn toggle_hidden(
        &mut self,
        cell: &mut Cell,
        commands: &mut Commands,
        color_query: &mut Query<&mut Handle<ColorMaterial>>,
        cell_colors: &CellColors,
        board: &mut Board,
        ev_cell_update: &mut EventWriter<CellUpdateEvent>,
    ) {
        if self.cell_type.is_none() {
            return;
        }
        self.hidden = !self.hidden;
        let (c1, c2) = if self.hidden {
            (
                cell_colors.yellow_light.clone(),
                cell_colors.yellow_medium.clone(),
            )
        } else {
            match self.cell_type.unwrap() {
                CellType::NumberCell(_) => (
                    cell_colors.gray_light.clone(),
                    cell_colors.gray_medium.clone(),
                ),
                CellType::EmptyCell => (
                    cell_colors.blue_light.clone(),
                    cell_colors.blue_medium.clone(),
                ),
            }
        };
        cell.click(commands, None, c1, c2, color_query);
        board.cells[cell.y as usize][cell.x as usize].1 = self.hidden;
        ev_cell_update.send(CellUpdateEvent);
    }
}

/// Component for the NumberCell type
#[derive(Debug, Component)]
pub struct NumberCell {
    pub count: u8,
    pub label: Entity,
    pub special_hint: bool,
}

/// Component for the EmptyCell type
#[derive(Debug, Component)]
pub struct EmptyCell;

#[derive(Debug, Component)]
pub struct UnsetCell;

pub struct CellUpdateEvent;
