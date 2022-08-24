use std::collections::VecDeque;

use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{
        default, shape::RegularPolygon, Assets, Color, Commands, Component, Handle, Mesh, ResMut,
        Transform, Visibility,
    },
    sprite::{ColorMaterial, ColorMesh2dBundle},
    text::{Text, Text2dBundle},
};
use interactable::{
    click::Clickable,
    hover::Hoverable,
    shapes::{Hexagon, Shape},
};

use crate::{
    components::{
        Cell, CellInner, CellOuter, CellType, ColumnHint, EmptyCell, HiddenCell, HintDirection,
        HintType, NumberCell,
    },
    constants::{RADIUS, Z_INDEX_CELL_BACK, Z_INDEX_CELL_INNER, Z_INDEX_CELL_OUTER, Z_INDEX_TEXT},
    functions::spawn_cell_text,
    resources::TextSettings,
};

/// Used to pass configuration from parser to board
pub struct BoardConfig {
    pub cells: Vec<Vec<(Option<CellType>, bool)>>,
    pub hints: Vec<ColumnHint>,
}

// TODO: Actually use this
/// Board component storing common variables
#[derive(Component)]
pub struct Board {
    pub cells: Vec<Option<Cell>>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    // TODO: make nicer
    /// An absolute monster of setup.
    pub fn new(
        commands: &mut Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        config: BoardConfig,
        text_settings: &TextSettings,
        white: Handle<ColorMaterial>,
        yellow: (Handle<ColorMaterial>, Handle<ColorMaterial>),
        gray: (Handle<ColorMaterial>, Handle<ColorMaterial>),
        blue: (Handle<ColorMaterial>, Handle<ColorMaterial>),
    ) -> Self {
        let cells = config.cells;
        let hints = config.hints;

        let medium_transform = Transform::from_translation(Vec3::new(0.0, 0.0, Z_INDEX_CELL_OUTER));
        let small_transform = Transform::from_translation(Vec3::new(0.0, 0.0, Z_INDEX_CELL_INNER));

        let big_hexagon = meshes.add(Mesh::from(RegularPolygon::new(RADIUS, 6)));
        let medium_hexagon = meshes.add(Mesh::from(RegularPolygon::new(RADIUS * 0.94, 6)));
        let small_hexagon = meshes.add(Mesh::from(RegularPolygon::new(RADIUS * 0.8, 6)));

        let mut cell_components = Vec::new();

        let height = cells.len();
        let width = cells[0].len();

        let w = ((width - 1) as f32 * RADIUS * 1.56) / 2.;
        let h = ((height - 1) as f32 * RADIUS * 1.8) / 2.;

        for y in 0..height {
            assert!(
                cells[y].len() == width,
                "All rows must have the same length."
            );
            for x in 0..width {
                let (cell_type, hidden) = cells[y][x];

                if cell_type.is_none() {
                    cell_components.push(None);
                    continue;
                }
                let cell_type = cell_type.unwrap();

                let tx = x as f32 * RADIUS * 1.56 - w;
                let ty = y as f32 * RADIUS * -1.8
                    + match x % 2 {
                        0 => 0.,
                        _ => RADIUS * 0.9,
                    }
                    + h;
                let colors = if !hidden {
                    match cell_type {
                        CellType::NumberCell(_) => gray.clone(),
                        CellType::EmptyCell => blue.clone(),
                    }
                } else {
                    yellow.clone()
                };

                let mut big_transform =
                    Transform::from_translation(Vec3::new(tx, ty, Z_INDEX_CELL_BACK));
                big_transform.rotate_z(f32::to_radians(90.0));

                let b1 = ColorMesh2dBundle {
                    mesh: medium_hexagon.clone().into(),
                    material: colors.0.into(),
                    transform: medium_transform,
                    ..default()
                };
                let b2 = ColorMesh2dBundle {
                    mesh: small_hexagon.clone().into(),
                    material: colors.1.into(),
                    transform: small_transform,
                    ..default()
                };

                // do the same for the child
                let child1 = commands.spawn_bundle(b1).insert(CellOuter).id();
                let child2 = commands.spawn_bundle(b2).insert(CellInner).id();

                let cell = commands
                    .spawn()
                    .insert_bundle(ColorMesh2dBundle {
                        mesh: big_hexagon.clone().into(),
                        material: white.clone().into(),
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
                        let text_entity = spawn_cell_text(big_transform, commands, count, &ts);
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
                        commands.entity(cell).insert(EmptyCell);
                    }
                }
                if hidden {
                    commands.entity(cell).insert_bundle(HiddenCell {
                        hoverable: Hoverable {
                            ignore_scale: true,
                            pass_through: false,
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
                    x,
                    y,
                    cell_type,
                    entity: cell,
                    outer_hexagon: child1,
                    inner_hexagon: child2,
                    orig: big_transform,
                    hovering: false,
                };
                // TODO: Rethink Cell type
                cell_components.push(Some(cell_component.clone()));
                commands.entity(cell).insert(cell_component);
            }
        }
        for mut hint in hints {
            let mut tx = hint.x as f32 * RADIUS * 1.56 - w;
            let mut ty = hint.y as f32 * RADIUS * -1.8
                + match hint.x % 2 {
                    0 => 0.,
                    _ => RADIUS * 0.9,
                }
                + h;
            let mut t = Transform::from_translation(Vec3::new(0., 0., Z_INDEX_TEXT));
            match hint.dir {
                HintDirection::TOP => (ty += 1.3 * RADIUS),
                HintDirection::LEFT => {
                    ty += RADIUS * 0.62;
                    tx -= RADIUS * 1.12;
                    t.rotate_z(1.047);
                }
                HintDirection::RIGHT => {
                    ty += RADIUS * 0.62;
                    tx += RADIUS * 1.12;
                    t.rotate_z(-1.047);
                }
            }
            t.translation.x = tx;
            t.translation.y = ty;
            let column = get_column(hint.x, hint.y, width, height, &cells, hint.dir);
            let count = count_empty_cells(&column);
            // TODO: Setting hint type and only reading it for style is unneccesary
            if hint.hint_type == HintType::SOME {
                hint.hint_type = match empty_connected(&column, count, false) {
                    true => HintType::CONNECTED,
                    false => HintType::SEPERATED,
                };
            }
            let mut ts = text_settings.clone();
            match hint.hint_type {
                HintType::CONNECTED => ts.style.color = Color::GREEN,
                HintType::SEPERATED => ts.style.color = Color::RED,
                _ => (),
            }

            commands.spawn_bundle(Text2dBundle {
                text: Text::from_section(format!("{}", count), ts.style)
                    .with_alignment(ts.alignment),
                transform: t,
                ..default()
            });
        }
        Self {
            cells: cell_components,
            width,
            height,
        }
    }
}

/// Get a (ordered?) list of neighbouring cells
fn get_neighbours(
    x: usize,
    y: usize,
    cells: &Vec<Vec<(Option<CellType>, bool)>>,
    w: usize,
    h: usize,
) -> Vec<(Option<CellType>, bool)> {
    let x = x as i32;
    let y = y as i32;
    let pos = if x % 2 == 0 {
        [
            (x, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
            (x, y + 1),
            (x - 1, y + 1),
            (x - 1, y),
        ]
    } else {
        [
            (x, y - 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x, y + 1),
            (x - 1, y),
            (x - 1, y - 1),
        ]
    };
    pos.iter()
        .filter(|(x, y)| !(*x < 0 || *x >= w as i32 || *y < 0 || *y >= h as i32))
        .map(|(x, y)| (cells[*y as usize][*x as usize]))
        .collect()
}

/// Get a (ordered?) list of cells in same column (or diagonal)
fn get_column(
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    cells: &Vec<Vec<(Option<CellType>, bool)>>,
    dir: HintDirection,
) -> Vec<(Option<CellType>, bool)> {
    match dir {
        HintDirection::TOP => (0..h).into_iter().map(|dy| cells[dy][x]).collect(),
        HintDirection::LEFT => {
            let mut pts = VecDeque::new();
            let mut dx = x;
            let mut dy = y;
            while dx > 0 && (dy > 0 || dx % 2 == 0) {
                if dx % 2 == 1 {
                    dy -= 1;
                }
                dx -= 1;
                pts.push_front(cells[dy][dx]);
            }
            pts.push_back(cells[y][x]);
            let mut dx = x;
            let mut dy = y;
            while dx < w - 1 && (dy < h - 1 || dx % 2 == 1) {
                if dx % 2 == 0 {
                    dy += 1;
                }
                dx += 1;
                pts.push_back(cells[dy][dx]);
            }
            pts.into()
        }
        HintDirection::RIGHT => {
            let mut pts = VecDeque::new();
            let mut dx = x;
            let mut dy = y;
            while dx > 0 && (dy < h - 1 || dx % 2 == 1) {
                if dx % 2 == 0 {
                    dy += 1;
                }
                dx -= 1;
                pts.push_front(cells[dy][dx]);
            }
            pts.push_back(cells[y][x]);
            let mut dx = x;
            let mut dy = y;
            while dx < w - 1 && (dy > 0 || dx % 2 == 0) {
                if dx % 2 == 1 {
                    dy -= 1;
                }
                dx += 1;
                pts.push_back(cells[dy][dx]);
            }
            pts.into()
        }
    }
}

/// Count how many cells in a list are empty
fn count_empty_cells(cells: &Vec<(Option<CellType>, bool)>) -> u8 {
    cells
        .iter()
        .map(|c| {
            if let Some(ct) = c.0 {
                if ct == CellType::EmptyCell {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum()
}

// TODO: So many clones...
/// Check if the empty cells are connected or seperated
fn empty_connected(cells: &Vec<(Option<CellType>, bool)>, count: u8, circular: bool) -> bool {
    if count == 0 {
        return true;
    }
    let mut cells = cells.clone();
    if circular {
        cells.extend(cells.clone());
    }
    let mut second_chance = circular;
    let mut remaining = count;
    let mut begun = false;
    for (ct, _h) in cells {
        if remaining == 0 {
            return true;
        }
        if begun {
            if let Some(ct) = ct {
                if ct == CellType::EmptyCell {
                    remaining -= 1;
                } else if second_chance {
                    second_chance = false;
                    remaining = count;
                    begun = false;
                } else {
                    break;
                }
            }
        } else {
            if let Some(ct) = ct {
                if ct == CellType::EmptyCell {
                    begun = true;
                    remaining -= 1;
                }
            }
        }
    }
    false
}
