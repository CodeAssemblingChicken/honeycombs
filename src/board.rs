use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{
        default, shape::RegularPolygon, Assets, Commands, Component, Handle, Mesh, ResMut,
        Transform,
    },
    sprite::{ColorMaterial, ColorMesh2dBundle},
};
use interactable::{
    click::Clickable,
    hover::Hoverable,
    shapes::{Hexagon, Shape},
};

use crate::{
    components::{
        Cell, CellInner, CellOuter, CellType, EmptyCell, HiddenCell, NumberCell, TextSettings,
    },
    constants::{RADIUS, Z_INDEX_CELL_BACK, Z_INDEX_CELL_INNER, Z_INDEX_CELL_OUTER},
    functions::spawn_cell_text,
};

#[derive(Component)]
pub struct Board {
    pub cells: Vec<Option<Cell>>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn new(
        commands: &mut Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        cells: Vec<Vec<(Option<CellType>, bool)>>,
        text_settings: &TextSettings,
        white: Handle<ColorMaterial>,
        yellow: (Handle<ColorMaterial>, Handle<ColorMaterial>),
        gray: (Handle<ColorMaterial>, Handle<ColorMaterial>),
        blue: (Handle<ColorMaterial>, Handle<ColorMaterial>),
    ) -> Self {
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
                let ty = y as f32 * RADIUS * 1.8
                    + match x % 2 {
                        0 => 0.,
                        _ => RADIUS * 0.9,
                    }
                    - h;
                let colors = if !hidden {
                    match cell_type {
                        CellType::NumberCell => gray.clone(),
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
                    CellType::NumberCell => {
                        let count = get_empty_neighbours(x as i32, y as i32, &cells, width, height);
                        let nc = NumberCell { count };
                        if !hidden {
                            spawn_cell_text(big_transform, commands, &nc, &text_settings);
                        }
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
        Self {
            cells: cell_components,
            width,
            height,
        }
    }
}

fn get_empty_neighbours(
    x: i32,
    y: i32,
    cells: &Vec<Vec<(Option<CellType>, bool)>>,
    w: usize,
    h: usize,
) -> u8 {
    let pos = if x % 2 == 0 {
        [
            (x - 1, y - 1),
            (x - 1, y),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y - 1),
            (x + 1, y),
        ]
    } else {
        [
            (x - 1, y),
            (x - 1, y + 1),
            (x, y - 1),
            (x, y + 1),
            (x + 1, y),
            (x + 1, y + 1),
        ]
    };

    pos.iter()
        .map(|(x, y)| {
            if x < &0 || x >= &(w as i32) || y < &0 || y >= &(h as i32) {
                0
            } else {
                if let Some(ct) = cells[*y as usize][*x as usize].0 {
                    if ct == CellType::EmptyCell {
                        1
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
        })
        .sum()
}
