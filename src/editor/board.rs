use super::{
    components::{EditorCell, EmptyCell, NumberCell, UnsetCell},
    functions::{row_empty, spawn_cell_common},
};
use crate::{
    components::{BoardConfig, CellType, HintType},
    functions::calc_dimensions,
    resources::{CellColors, CellMeshes, TextSettings},
};
use bevy::prelude::{Commands, Visibility};

pub struct Board {
    pub cells: Vec<Vec<(Option<CellType>, bool)>>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn new(
        commands: &mut Commands,
        config: &BoardConfig,
        cell_meshes: &CellMeshes,
        cell_colors: &CellColors,
        text_settings: &TextSettings,
    ) -> Self {
        let cells = &config.cells;
        let hints = &config.hints;
        let width = config.width;
        let height = config.height;

        let (w, h) = calc_dimensions(width, height);

        for (y, row) in cells.iter().enumerate() {
            for (x, (ct, hidden)) in row.iter().enumerate() {
                let x = x as i32;
                let y = y as i32;
                let cell = commands.spawn().id();
                let colors = match ct {
                    Some(CellType::EmptyCell) => {
                        commands.entity(cell).insert(EmptyCell);
                        if *hidden {
                            (
                                cell_colors.white.clone(),
                                cell_colors.yellow_medium.clone(),
                                cell_colors.yellow_light.clone(),
                            )
                        } else {
                            (
                                cell_colors.white.clone(),
                                cell_colors.blue_medium.clone(),
                                cell_colors.blue_light.clone(),
                            )
                        }
                    }
                    Some(CellType::NumberCell(_)) => {
                        if *hidden {
                            (
                                cell_colors.white.clone(),
                                cell_colors.yellow_medium.clone(),
                                cell_colors.yellow_light.clone(),
                            )
                        } else {
                            (
                                cell_colors.white.clone(),
                                cell_colors.gray_medium.clone(),
                                cell_colors.gray_light.clone(),
                            )
                        }
                    }

                    None => {
                        commands.entity(cell).insert(UnsetCell);
                        (
                            cell_colors.alpha0.clone(),
                            cell_colors.alpha1.clone(),
                            cell_colors.alpha0.clone(),
                        )
                    }
                };
                let text_entity = spawn_cell_common(
                    commands,
                    cell,
                    (cell_meshes, text_settings),
                    colors,
                    (x, y),
                    (w, h),
                    (true, true, true),
                );
                commands.entity(cell).insert(EditorCell {
                    hidden: *hidden,
                    cell_type: *ct,
                    text_entity,
                });
                if let Some(CellType::NumberCell(ht)) = ct {
                    commands.entity(cell).insert(NumberCell {
                        count: 0,
                        label: text_entity,
                        special_hint: ht != &HintType::None,
                    });
                    commands
                        .entity(text_entity)
                        .insert(Visibility { is_visible: true });
                }
            }
        }
        Self {
            cells: cells.clone(),
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
