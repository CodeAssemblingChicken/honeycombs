use crate::{
    board_functions::{count_empty_cells, empty_connected, get_neighbours},
    components::{Cell, CellInner, CellOuter, CellType, ColumnHint, HintType},
    constants::{INNER_TRANSFORM, OUTER_TRANSFORM, RADIUS, Z_INDEX_CELL_BACK},
    functions::{calc_translation, spawn_cell_text, spawn_hint},
    level::components::{EmptyCell, GameCell, HiddenCell, NumberCell},
    resources::{CellColors, CellMeshes, TextSettings},
};
use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{default, Color, Commands, Entity, Transform, Visibility},
    sprite::ColorMesh2dBundle,
};
use interactable::{
    click::Clickable,
    hover::Hoverable,
    shapes::{Hexagon, Shape},
};

/// Used to pass configuration from parser to board
pub struct BoardConfig {
    pub cells: Vec<Vec<(Option<CellType>, bool)>>,
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
        let hints = config.hints;

        let mut cell_entities = Vec::new();
        let mut text_entities = Vec::new();

        let height = cells.len();
        let width = cells[0].len();

        let w = ((width - 1) as f32 * RADIUS * 1.56) / 2.;
        let h = ((height - 1) as f32 * RADIUS * 1.8) / 2.;

        let mut blues_remaining = 0;

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

                let b1 = ColorMesh2dBundle {
                    mesh: cell_meshes.medium_hexagon.clone().into(),
                    material: colors.0,
                    transform: OUTER_TRANSFORM,
                    ..default()
                };
                let b2 = ColorMesh2dBundle {
                    mesh: cell_meshes.small_hexagon.clone().into(),
                    material: colors.1,
                    transform: INNER_TRANSFORM,
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

                match cell_type {
                    CellType::NumberCell(mut ht) => {
                        let neighbours = get_neighbours(x, y, &cells, width, height);
                        let count = count_empty_cells(&neighbours);
                        if ht == HintType::SOME {
                            ht = match empty_connected(&neighbours, count, true) {
                                true => HintType::CONNECTED,
                                false => HintType::SEPERATED,
                            };
                        }
                        let mut ts = text_settings.clone();
                        match ht {
                            HintType::CONNECTED => ts.style.color = Color::GREEN,
                            HintType::SEPERATED => ts.style.color = Color::RED,
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
                            blues_remaining += 1;
                        }
                        commands.entity(cell).insert(EmptyCell);
                    }
                }
                if hidden {
                    commands.entity(cell).insert_bundle(HiddenCell {
                        hoverable: Hoverable {
                            ignore_scale: true,
                            shape: Shape::Hexagon(Hexagon {
                                radius: RADIUS,
                                point_up: false,
                            }),
                            ..default()
                        },
                        clickable: Clickable {
                            ignore_scale: true,
                            shape: Shape::Hexagon(Hexagon {
                                radius: RADIUS,
                                point_up: false,
                            }),
                            left_released: true,
                            right_released: true,

                            ..default()
                        },
                    });
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
            remaining: blues_remaining,
        }
    }
}
