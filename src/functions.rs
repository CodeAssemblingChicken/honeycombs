use std::{env, path::PathBuf};

use crate::{
    board_functions::{count_empty_cells, empty_connected, get_column},
    components::{CellInner, CellOuter, ColumnHint},
    constants::{INNER_TRANSFORM, OUTER_TRANSFORM, RADIUS, Z_INDEX_TEXT},
    enums::{CellType, HintDirection, HintType},
    resources::{LoadState, TextSettings},
    states::AppState,
};
use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{default, Color, Commands, Entity, Handle, Mesh, State, Transform},
    sprite::{ColorMaterial, ColorMesh2dBundle},
    text::{Text, Text2dBundle, TextAlignment, TextStyle},
};
use interactable::{
    components::Interactable,
    shapes::{Hexagon, Shape},
};

pub fn make_cell_interactable(commands: &mut Commands, cell: Entity, radius: f32) {
    commands.entity(cell).insert(Interactable {
        shape: Shape::Hexagon(Hexagon {
            radius,
            point_up: false,
        }),
        ..default()
    });
}

pub fn spawn_cell(
    commands: &mut Commands,
    cell: Entity,
    (m1, m2, m3): (Handle<Mesh>, Handle<Mesh>, Handle<Mesh>),
    (c1, c2, c3): (
        Handle<ColorMaterial>,
        Handle<ColorMaterial>,
        Handle<ColorMaterial>,
    ),
    transform: Transform,
) -> (Entity, Entity) {
    let b1 = ColorMesh2dBundle {
        mesh: m2.into(),
        material: c2,
        transform: OUTER_TRANSFORM,
        ..default()
    };
    let b2 = ColorMesh2dBundle {
        mesh: m3.into(),
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
            mesh: m1.into(),
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
    text_style: TextStyle,
    text_alignment: TextAlignment,
) -> Entity {
    let mut t = Transform::identity();
    t.translation.z = Z_INDEX_TEXT;
    t.rotate_z(f32::to_radians(-90.0));
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(text, text_style).with_alignment(text_alignment),
            transform: t,
            ..default()
        })
        .id()
}

pub fn spawn_hint(
    commands: &mut Commands,
    mut hint: ColumnHint,
    cells: &[Vec<(Option<CellType>, bool)>],
    text_settings: &TextSettings,
    (w, h): (f32, f32),
    (width, height): (usize, usize),
) -> (Entity, f32) {
    let (mut tx, mut ty) = calc_translation(hint.x as i32, hint.y as i32, w, h);
    let mut t = Transform::from_xyz(0., 0., Z_INDEX_TEXT);
    match hint.dir {
        HintDirection::Down => (ty += 1.3 * RADIUS),
        HintDirection::LeftDown => {
            ty += RADIUS * 0.62;
            tx += RADIUS * 1.12;
            t.rotate_z(-1.047);
        }
        HintDirection::RightDown => {
            ty += RADIUS * 0.62;
            tx -= RADIUS * 1.12;
            t.rotate_z(1.047);
        }
        HintDirection::Up => {
            ty -= 1.3 * RADIUS;
            t.rotate_z(std::f32::consts::PI);
        }
        HintDirection::LeftUp => {
            ty -= RADIUS * 0.62;
            tx += RADIUS * 1.12;
            t.rotate_z(-2.097);
        }
        HintDirection::RightUp => {
            ty -= RADIUS * 0.62;
            tx -= RADIUS * 1.12;
            t.rotate_z(2.097);
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
        HintType::Connected => ts.style_cell.color = Color::GREEN,
        HintType::Seperated => ts.style_cell.color = Color::rgb(1.0, 0.2, 0.2),
        _ => (),
    }

    let trimmed_col_len: u16 = column
        .iter()
        .skip_while(|(ct, _)| ct.is_none())
        .collect::<Vec<&(Option<CellType>, bool)>>()
        .iter()
        .rev()
        .skip_while(|(ct, _)| ct.is_none())
        .map(|_| 1)
        .sum();

    (
        commands
            .spawn_bundle(Text2dBundle {
                text: Text::from_section(format!("{}", count), ts.style_cell)
                    .with_alignment(ts.alignment),
                transform: t,
                ..default()
            })
            .insert(hint)
            .id(),
        trimmed_col_len as f32 * RADIUS * 1.8,
    )
}

pub fn rescale_board(
    board_width: usize,
    board_height: usize,
    margin: usize,
    wd_width: f32,
    wd_height: f32,
    // camera_query: &mut Query<&mut Transform, With<Camera>>,
    root: &mut Transform,
) {
    let w = 1. / (((board_width + margin) as f32 * RADIUS * 1.56) / wd_width);
    let h = 1. / (((board_height + margin) as f32 * RADIUS * 1.8) / wd_height);
    let s = w.min(h);
    root.scale = Vec3::new(s, s, 1.0);
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

/// Switch to a new state replacing the full stack.
/// This is relevant for the Overlay state.
pub fn switch_state(
    next_state: Option<AppState>,
    app_state: &mut State<AppState>,
    mut load_state: &mut LoadState,
) {
    load_state.next_state = next_state;
    app_state.replace(AppState::StateChange).unwrap();
}

/// Returns the base path of the assets directory, which is normally the executable's parent
/// directory.
///
/// If the `CARGO_MANIFEST_DIR` environment variable is set, then its value will be used
/// instead. It's set by cargo when running with `cargo run`.
pub fn get_base_path() -> PathBuf {
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        PathBuf::from(manifest_dir)
    } else {
        env::current_exe()
            .map(|path| {
                path.parent()
                    .map(|exe_parent_path| exe_parent_path.to_owned())
                    .unwrap()
            })
            .unwrap()
    }
}
