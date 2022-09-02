use crate::{
    board_functions::{count_empty_cells, empty_connected, get_neighbours},
    components::{BoardConfig, Cell, CellType, HintType},
    constants::{RADIUS, Z_INDEX_CELL_BACK, Z_INDEX_TEXT},
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
    prelude::{default, Color, Commands, Entity, Transform, Visibility},
    text::{Text, Text2dBundle},
};

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
        config: &BoardConfig,
        text_settings: &TextSettings,
        cell_meshes: &CellMeshes,
        cell_colors: &CellColors,
    ) -> Self {
        let cells = &config.cells;
        let hints = &config.hints;
        let width = config.width;
        let height = config.height;

        let mut cell_entities = Vec::new();
        let mut text_entities = Vec::new();

        let (w, h) = calc_dimensions(width, height);

        let mut empty_remaining = 0;

        for y in 0..height {
            for x in 0..width {
                let (cell_type, hidden) = cells[y][x];

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
                    (
                        cell_meshes.std_hexagon_back.clone(),
                        cell_meshes.std_hexagon_outer.clone(),
                        cell_meshes.std_hexagon_inner.clone(),
                    ),
                    (cell_colors.white.clone(), colors.0, colors.1),
                    big_transform,
                );

                match cell_type {
                    CellType::NumberCell(mut ht) => {
                        let neighbours = get_neighbours(x as i32, y as i32, cells, width, height);
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
                    make_cell_interactable(
                        commands,
                        cell,
                        interactable::click::MouseActions {
                            left_released: true,
                            right_released: true,
                            ..default()
                        },
                        RADIUS,
                    );
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
                *hint,
                cells,
                text_settings,
                (w, h),
                (width, height),
            ));
        }
        if let Some(text) = &config.text {
            commands.spawn_bundle(Text2dBundle {
                text: Text::from_section(format!("{}", text), text_settings.style.clone())
                    .with_alignment(text_settings.alignment),
                transform: Transform::from_translation(Vec3::new(
                    0.,
                    -h - 3. * RADIUS,
                    Z_INDEX_TEXT,
                )),
                ..default()
            });
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
