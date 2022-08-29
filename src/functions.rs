use crate::{
    board_functions::{count_empty_cells, empty_connected, get_column},
    components::{
        CellInner, CellOuter, CellType, ColumnHint, HintDirection, HintType, InteractableCell,
    },
    constants::{INNER_TRANSFORM, OUTER_TRANSFORM, RADIUS, Z_INDEX_TEXT},
    resources::{CellMeshes, TextSettings},
};
use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{default, Camera, Color, Commands, Entity, Handle, Query, Transform, With},
    sprite::{ColorMaterial, ColorMesh2dBundle},
    text::{Text, Text2dBundle},
};
use interactable::{
    click::Clickable,
    hover::Hoverable,
    shapes::{Hexagon, Shape},
};

pub fn make_cell_interactable(
    commands: &mut Commands,
    cell: Entity,
    (left_released, right_released, middle_released): (bool, bool, bool),
) {
    commands.entity(cell).insert_bundle(InteractableCell {
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
            left_released,
            right_released,
            middle_released,
            ..default()
        },
    });
}

pub fn spawn_cell(
    commands: &mut Commands,
    cell: Entity,
    cell_meshes: &CellMeshes,
    (c1, c2, c3): (
        Handle<ColorMaterial>,
        Handle<ColorMaterial>,
        Handle<ColorMaterial>,
    ),
    transform: Transform,
) -> (Entity, Entity) {
    let b1 = ColorMesh2dBundle {
        mesh: cell_meshes.medium_hexagon.clone().into(),
        material: c2,
        transform: OUTER_TRANSFORM,
        ..default()
    };
    let b2 = ColorMesh2dBundle {
        mesh: cell_meshes.small_hexagon.clone().into(),
        material: c3,
        transform: INNER_TRANSFORM,
        ..default()
    };

    // do the same for the child
    let child1 = commands.spawn_bundle(b1).insert(CellOuter).id();
    let child2 = commands.spawn_bundle(b2).insert(CellInner).id();

    commands
        .entity(cell)
        .insert_bundle(ColorMesh2dBundle {
            mesh: cell_meshes.big_hexagon.clone().into(),
            material: c1,
            transform,
            ..default()
        })
        .push_children(&[child1, child2]);
    (child1, child2)
}

/// Spawns the text in a cell
pub fn spawn_cell_text(
    commands: &mut Commands,
    text: &str,
    text_settings: &TextSettings,
) -> Entity {
    let mut t = Transform::identity();
    t.translation.z = Z_INDEX_TEXT;
    t.rotate_z(f32::to_radians(-90.0));
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(text, text_settings.style.clone())
                .with_alignment(text_settings.alignment),
            transform: t,
            ..default()
        })
        .id()
}

pub fn spawn_hint(
    commands: &mut Commands,
    mut hint: ColumnHint,
    cells: &[Vec<Option<CellType>>],
    text_settings: &TextSettings,
    (w, h): (f32, f32),
    (width, height): (usize, usize),
) -> Entity {
    let (mut tx, mut ty) = calc_translation(hint.x as i32, hint.y as i32, w, h);
    let mut t = Transform::from_translation(Vec3::new(0., 0., Z_INDEX_TEXT));
    match hint.dir {
        HintDirection::Top => (ty += 1.3 * RADIUS),
        HintDirection::Left => {
            ty += RADIUS * 0.62;
            tx -= RADIUS * 1.12;
            t.rotate_z(1.047);
        }
        HintDirection::Right => {
            ty += RADIUS * 0.62;
            tx += RADIUS * 1.12;
            t.rotate_z(-1.047);
        }
    }
    t.translation.x = tx;
    t.translation.y = ty;
    let column = get_column(hint.x, hint.y, width, height, cells, hint.dir);
    let count = count_empty_cells(&column);
    // TODO: Setting hint type and only reading it for style is unneccesary
    if hint.hint_type == HintType::Some {
        hint.hint_type = match empty_connected(&column, count, false) {
            true => HintType::Connected,
            false => HintType::Seperated,
        };
    }
    let mut ts = text_settings.clone();
    match hint.hint_type {
        HintType::Connected => ts.style.color = Color::GREEN,
        HintType::Seperated => ts.style.color = Color::RED,
        _ => (),
    }

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(format!("{}", count), ts.style).with_alignment(ts.alignment),
            transform: t,
            ..default()
        })
        .id()
}

pub fn rescale_board(
    board_width: usize,
    board_height: usize,
    margin: usize,
    wd_width: f32,
    wd_height: f32,
    camera_query: &mut Query<&mut Transform, With<Camera>>,
) {
    let w = ((board_width + margin) as f32 * RADIUS * 1.56) / wd_width;
    let h = ((board_height + margin) as f32 * RADIUS * 1.8) / wd_height;
    let s = w.max(h);
    for mut t in camera_query.iter_mut() {
        t.scale = Vec3::new(s, s, 1.0);
    }
}

pub fn calc_translation(x: i32, y: i32, w: f32, h: f32) -> (f32, f32) {
    let tx = x as f32 * RADIUS * 1.56 - w;
    let ty = y as f32 * RADIUS * -1.8
        + match x % 2 {
            0 => 0.,
            _ => RADIUS * 0.9,
        }
        + h;
    (tx, ty)
}

pub fn calc_dimensions(width: usize, height: usize) -> (f32, f32) {
    let w = ((width - 1) as f32 * RADIUS * 1.56) / 2.;
    let h = ((height - 1) as f32 * RADIUS * 1.8) / 2.;
    (w, h)
}
