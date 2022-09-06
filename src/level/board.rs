use super::components::{MistakesText, RemainingText};
use crate::{
    board_functions::{count_empty_cells, empty_connected, get_neighbours},
    components::{BoardConfig, Cell, CellType, HintType, RootComponent, TextSectionConfig},
    constants::{GameColor, RADIUS, Z_INDEX_CELL_BACK, Z_INDEX_TEXT},
    functions::{
        calc_dimensions, calc_translation, make_cell_interactable, spawn_cell, spawn_cell_text,
        spawn_hint,
    },
    level::components::{EmptyCell, GameCell, NumberCell},
    resources::{CellMeshes, GameColors, TextSettings},
};
use bevy::{
    hierarchy::BuildChildren,
    math::Vec2,
    prelude::{
        default, shape::Quad, Assets, Color, Commands, Entity, Mesh, SpatialBundle, Transform,
        Visibility,
    },
    sprite::{ColorMaterial, ColorMesh2dBundle},
    text::{Text, Text2dBundle},
};
use interactable::{
    click::{Clickable, MouseActions},
    shapes::Shape,
};

/// Board component storing common variables
pub struct Board {
    pub cells: Vec<Entity>,
    pub texts: Vec<Entity>,
    pub width: usize,
    pub height: usize,
    remaining: (u16, u16),
    hidden: u16,
    mistakes: u16,
    stage_id: u8,
    level_id: u8,
}

impl Board {
    // TODO: make nicer
    /// An absolute monster of setup.
    pub fn new(
        commands: &mut Commands,
        root_transform: Transform,
        config: &BoardConfig,
        (cell_meshes, game_colors, text_settings): (&CellMeshes, &GameColors, &TextSettings),
        (stage_id, level_id): (u8, u8),
        (meshes, colors): (&mut Assets<Mesh>, &mut Assets<ColorMaterial>),
    ) -> Self {
        let cells = &config.cells;
        let hints = &config.hints;
        let width = config.width;
        let height = config.height;

        let mut cell_entities = Vec::new();
        let mut text_entities = Vec::new();

        let (w, h) = calc_dimensions(width, height);

        let mut empty_remaining = 0;
        let mut number_remaining = 0;

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
                            game_colors.gray_medium.clone(),
                            game_colors.gray_light.clone(),
                        ),
                        CellType::EmptyCell => (
                            game_colors.blue_medium.clone(),
                            game_colors.blue_light.clone(),
                        ),
                    }
                } else {
                    (
                        game_colors.yellow_medium.clone(),
                        game_colors.yellow_light.clone(),
                    )
                };

                let mut big_transform = Transform::from_xyz(tx, ty, Z_INDEX_CELL_BACK);
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
                    (game_colors.white.clone(), colors.0, colors.1),
                    big_transform,
                );

                match cell_type {
                    CellType::NumberCell(mut ht) => {
                        if hidden {
                            number_remaining += 1;
                        }
                        let neighbours = get_neighbours(x as i32, y as i32, cells, width, height);
                        let count = count_empty_cells(&neighbours);
                        if ht == HintType::Some {
                            ht = match empty_connected(&neighbours, count, true) {
                                true => HintType::Connected,
                                false => HintType::Seperated,
                            };
                        }
                        let mut ts = text_settings.style_cell.clone();
                        match ht {
                            HintType::Connected => ts.color = Color::GREEN,
                            HintType::Seperated => ts.color = Color::rgb(1.0, 0.2, 0.2),
                            _ => (),
                        }
                        let text_entity = spawn_cell_text(
                            commands,
                            &format!("{}", count),
                            ts,
                            text_settings.alignment,
                        );
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
                commands
                    .entity(cell)
                    .insert(cell_component)
                    .insert(GameCell { cell_type, hidden });
                cell_entities.push(cell);
            }
        }
        let line_color = colors.add(ColorMaterial::from(Color::rgba(1.0, 1.0, 1.0, 0.4)));
        for hint in hints {
            let (hint_entity, length) = spawn_hint(
                commands,
                *hint,
                cells,
                text_settings,
                (w, h),
                (width, height),
            );
            let hint_line = commands
                .spawn_bundle(ColorMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(Quad::new(Vec2::new(16.0, length))))
                        .into(),
                    material: line_color.clone(),
                    transform: Transform::from_xyz(0., -(length) / 2. - RADIUS / 3., 2.),
                    visibility: Visibility { is_visible: false },
                    ..default()
                })
                .id();
            commands
                .entity(hint_entity)
                .insert(Clickable {
                    shape: Shape::Quad(interactable::shapes::Quad {
                        width: 0.6 * RADIUS,
                        height: 0.6 * RADIUS,
                    }),
                    mouse_actions: MouseActions {
                        left_released: true,
                        ..default()
                    },
                    ..default()
                })
                .add_child(hint_line);
            text_entities.push(hint_entity);
        }
        let texts = [
            TextSectionConfig::new("Hello ", None, false),
            TextSectionConfig::new("World", Some(GameColor::BLUE_LIGHT), false),
            TextSectionConfig::new("!\nOK", None, false),
        ];

        let text1 = if let Some(text) = &config.text {
            Some(
                commands
                    .spawn_bundle(Text2dBundle {
                        text: Text::from_sections(
                            // texts
                            //     .iter()
                            //     .map(|tsc| tsc.to_text_section(&text_settings.style_cell)),
                            text.2
                                .iter()
                                .map(|tsc| tsc.to_text_section(&text_settings.style_cell)),
                        )
                        .with_alignment(text_settings.alignment),
                        transform: Transform::from_xyz(0., -h - 3. * RADIUS, Z_INDEX_TEXT),
                        ..default()
                    })
                    .id(),
            )
        } else {
            None
        };

        let text2 = commands
            .spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    format!("{}: {}", "Remaining", empty_remaining),
                    text_settings.style_cell.clone(),
                )
                .with_alignment(text_settings.alignment),
                transform: Transform::from_xyz(w + 3. * RADIUS, h, Z_INDEX_TEXT),
                ..default()
            })
            .insert(RemainingText)
            .id();
        let text3 = commands
            .spawn_bundle(Text2dBundle {
                text: Text::from_section(
                    format!("{}: {}", "Mistakes", empty_remaining),
                    text_settings.style_cell.clone(),
                )
                .with_alignment(text_settings.alignment),
                transform: Transform::from_xyz(w + 3. * RADIUS, h - RADIUS, Z_INDEX_TEXT),
                ..default()
            })
            .insert(MistakesText)
            .id();

        let root = commands
            .spawn()
            .push_children(&cell_entities)
            .push_children(&text_entities)
            .push_children(&[text2, text3])
            .insert_bundle(SpatialBundle::from_transform(root_transform))
            .insert(RootComponent)
            .id();
        if let Some(t) = text1 {
            commands.entity(root).add_child(t);
        }

        Self {
            cells: cell_entities,
            texts: text_entities,
            width,
            height,
            remaining: (empty_remaining, number_remaining),
            hidden: empty_remaining + number_remaining,
            mistakes: 0,
            stage_id,
            level_id,
        }
    }
    pub fn uncover_empty(&mut self) {
        if self.get_empty_remaining() > u16::MIN {
            self.remaining.0 -= 1;
        }
    }
    pub fn uncover_number(&mut self) {
        if self.get_number_remaining() > u16::MIN {
            self.remaining.1 -= 1;
        }
    }
    pub fn make_mistake(&mut self) {
        if self.mistakes < u16::MAX {
            self.mistakes += 1;
        }
    }
    pub fn is_solved(&self) -> bool {
        self.get_empty_remaining() == 0 && self.get_number_remaining() == 0
    }
    pub fn get_max_points(&self) -> u16 {
        ((self.get_hidden() as f32).powf(0.6) as u16).max(1).min(30)
    }
    pub fn get_points(&self) -> u16 {
        self.get_max_points().saturating_sub(self.mistakes)
    }
    pub fn get_empty_remaining(&self) -> u16 {
        self.remaining.0
    }
    pub fn get_number_remaining(&self) -> u16 {
        self.remaining.1
    }
    pub fn get_hidden(&self) -> u16 {
        self.hidden
    }
    pub fn get_mistakes(&self) -> u16 {
        self.mistakes
    }
    pub fn get_stage_id(&self) -> u8 {
        self.stage_id
    }
    pub fn get_level_id(&self) -> u8 {
        self.level_id
    }
}
