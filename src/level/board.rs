use crate::{
    board_functions::{count_empty_cells, empty_connected, get_neighbours},
    components::{Cell, CellType, ColumnHint, HintType},
    constants::Z_INDEX_CELL_BACK,
    functions::{
        calc_dimensions, calc_translation, make_cell_interactable, spawn_cell, spawn_cell_text,
        spawn_hint,
    },
    level::components::{EmptyCell, GameCell, NumberCell},
    resources::{CellColors, CellMeshes, TextSettings},
};
use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{Color, Commands, Entity, Transform, Visibility},
};

/// Used to pass configuration from parser to board
pub struct BoardConfig {
    pub cells: Vec<Vec<Option<CellType>>>,
    pub hiddens: Vec<Vec<bool>>,
    pub hints: Vec<ColumnHint>,
}

// TODO: Actually use this
/// Board component storing common variables
pub struct Board {
    pub cells: Vec<Entity>,
    pub texts: Vec<Entity>,
    pub width: usize,
    pub height: usize,
    pub remaining: usize,
}

impl Board {
    // TODO: make nicer
    /// An absolute monster of setup.
    pub fn new(
        commands: &mut Commands,
        config: BoardConfig,
        text_settings: &TextSettings,
        cell_meshes: &CellMeshes,
        cell_colors: &CellColors,
    ) -> Self {
        let cells = config.cells;
        let hiddens = config.hiddens;
        let hints = config.hints;

        let mut cell_entities = Vec::new();
        let mut text_entities = Vec::new();

        let height = cells.len();
        let width = cells[0].len();

        let (w, h) = calc_dimensions(width, height);

        let mut empty_remaining = 0;

        for y in 0..height {
            for x in 0..width {
                let cell_type = cells[y][x];
                let hidden = hiddens[y][x];

                if cell_type.is_none() {
                    continue;
                }
                let cell_type = cell_type.unwrap();

                let (tx, ty) = calc_translation(x as i32, y as i32, w, h);
                let colors = if !hidden {
                    match cell_type {
                        CellType::NumberCell(_) => (
                            cell_colors.gray_medium.clone(),
                            cell_colors.gray_light.clone(),
                        ),
                        CellType::EmptyCell => (
                            cell_colors.blue_medium.clone(),
                            cell_colors.blue_light.clone(),
                        ),
                    }
                } else {
                    (
                        cell_colors.yellow_medium.clone(),
                        cell_colors.yellow_light.clone(),
                    )
                };

                let mut big_transform =
                    Transform::from_translation(Vec3::new(tx, ty, Z_INDEX_CELL_BACK));
                big_transform.rotate_z(f32::to_radians(90.0));

                let cell = commands.spawn().id();

                let (child1, child2) = spawn_cell(
                    commands,
                    cell,
                    cell_meshes,
                    (cell_colors.white.clone(), colors.0, colors.1),
                    big_transform,
                );

                match cell_type {
                    CellType::NumberCell(mut ht) => {
                        let neighbours = get_neighbours(x as i32, y as i32, &cells, width, height);
                        let count = count_empty_cells(&neighbours);
                        if ht == HintType::Some {
                            ht = match empty_connected(&neighbours, count, true) {
                                true => HintType::Connected,
                                false => HintType::Seperated,
                            };
                        }
                        let mut ts = text_settings.clone();
                        match ht {
                            HintType::Connected => ts.style.color = Color::GREEN,
                            HintType::Seperated => ts.style.color = Color::RED,
                            _ => (),
                        }
                        let text_entity = spawn_cell_text(commands, &format!("{}", count), &ts);
                        commands.entity(cell).add_child(text_entity);
                        if hidden {
                            commands
                                .entity(text_entity)
                                .insert(Visibility { is_visible: false });
                        }
                        let nc = NumberCell {
                            count,
                            label: text_entity,
                        };
                        commands.entity(cell).insert(nc);
                    }
                    CellType::EmptyCell => {
                        if hidden {
                            empty_remaining += 1;
                        }
                        commands.entity(cell).insert(EmptyCell);
                    }
                }
                if hidden {
                    make_cell_interactable(commands, cell, (true, true, false));
                }

                let cell_component = Cell {
                    x: x as i32,
                    y: y as i32,
                    entity: cell,
                    outer_hexagon: child1,
                    inner_hexagon: child2,
                    orig: big_transform,
                    hovering: false,
                };
                // TODO: Rethink Cell type
                commands
                    .entity(cell)
                    .insert(cell_component)
                    .insert(GameCell { cell_type });
                cell_entities.push(cell);
            }
        }
        for hint in hints {
            text_entities.push(spawn_hint(
                commands,
                hint,
                &cells,
                text_settings,
                (w, h),
                (width, height),
            ));
        }
        Self {
            cells: cell_entities,
            texts: text_entities,
            width,
            height,
            remaining: empty_remaining,
        }
    }
}
