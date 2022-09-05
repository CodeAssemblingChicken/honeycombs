use super::{
    components::{EditorCell, EmptyCell, NumberCell, UnsetCell},
    functions::{row_empty, spawn_cell_common},
};
use crate::{
    components::{BoardConfig, CellType, HintType, RootComponent},
    functions::calc_dimensions,
    resources::{CellMeshes, GameColors, TextSettings},
};
use bevy::{
    hierarchy::BuildChildren,
    prelude::{Commands, SpatialBundle, Transform, Visibility},
};

pub struct Board {
    pub cells: Vec<Vec<(Option<CellType>, bool)>>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn new(
        commands: &mut Commands,
        root_transform: Transform,
        config: &BoardConfig,
        cell_meshes: &CellMeshes,
        game_colors: &GameColors,
        text_settings: &TextSettings,
    ) -> Self {
        let cells = &config.cells;
        let _hints = &config.hints;
        let width = config.width;
        let height = config.height;

        let mut cell_entities = Vec::new();

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
                                game_colors.white.clone(),
                                game_colors.yellow_medium.clone(),
                                game_colors.yellow_light.clone(),
                            )
                        } else {
                            (
                                game_colors.white.clone(),
                                game_colors.blue_medium.clone(),
                                game_colors.blue_light.clone(),
                            )
                        }
                    }
                    Some(CellType::NumberCell(_)) => {
                        if *hidden {
                            (
                                game_colors.white.clone(),
                                game_colors.yellow_medium.clone(),
                                game_colors.yellow_light.clone(),
                            )
                        } else {
                            (
                                game_colors.white.clone(),
                                game_colors.gray_medium.clone(),
                                game_colors.gray_light.clone(),
                            )
                        }
                    }

                    None => {
                        commands.entity(cell).insert(UnsetCell);
                        (
                            game_colors.alpha0.clone(),
                            game_colors.alpha1.clone(),
                            game_colors.alpha0.clone(),
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
                cell_entities.push(cell);
            }
        }
        commands
            .spawn()
            .push_children(&cell_entities)
            .insert_bundle(SpatialBundle::from_transform(root_transform))
            .insert(RootComponent);

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
            if cells.len() < 2 || !row_empty(&cells[0]) {
                break;
            }
            // Remove from top
            cells.remove(0);
        }
        loop {
            if cells.len() < 2 || !row_empty(&cells[cells.len() - 1]) {
                break;
            }
            // Remove from bottom
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
            // Remove from left
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
            // Remove from right
            for row in &mut cells {
                *row = row.clone().into_iter().take(row.len() - 1).collect();
            }
        }
        // Perform left shift if necessary
        if row_empty(&(&cells).iter().map(|row| row[0]).collect()) {
            // Remove from left
            for row in &mut cells {
                *row = row.clone().into_iter().skip(1).collect();
            }
            if row_empty(&(&cells[0]).iter().step_by(2).map(|e| *e).collect()) {
                // Pull up every 2n
                println!("Shiftable 1");
                let h = cells.len() - 1;
                for x in (0..cells[0].len()).step_by(2) {
                    for y in 0..h {
                        cells[y][x] = cells[y + 1][x];
                    }
                    cells[h][x] = (None, false);
                }
            } else {
                // Insert new empty row, needed in some cases
                cells.push(vec![(None, false); cells[0].len()]);
                // Push down every 2n+1
                let h = cells.len() - 1;
                for x in (1..cells[0].len()).step_by(2) {
                    for y in (1..=h).rev() {
                        cells[y][x] = cells[y - 1][x];
                    }
                    cells[0][x] = (None, false);
                }
            }

            // Remove from bottom if necessary
            if cells.len() > 1 && row_empty(&cells[cells.len() - 1]) {
                cells.remove(cells.len() - 1);
            }
        }
        cells
    }
}
