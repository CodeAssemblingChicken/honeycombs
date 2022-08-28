use bevy::{
    hierarchy::BuildChildren,
    math::Vec3,
    prelude::{default, Camera, Commands, Entity, Handle, Query, Res, Transform, With},
    sprite::{ColorMaterial, ColorMesh2dBundle},
    window::Windows,
};

use crate::{
    components::{Cell, CellInner, CellOuter},
    constants::{INNER_TRANSFORM, OUTER_TRANSFORM, RADIUS, Z_INDEX_CELL_BACK},
    editor::components::UnsetCell,
    functions::{make_cell_interactable, rescale_board},
    resources::{CellColors, CellMeshes},
};

pub fn setup(
    mut commands: Commands,
    wnds: Res<Windows>,
    cell_meshes: Res<CellMeshes>,
    cell_colors: Res<CellColors>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    let width = 15;
    let height = 10;

    let w = ((width - 1) as f32 * RADIUS * 1.56) / 2.;
    let h = ((height - 1) as f32 * RADIUS * 1.8) / 2.;

    for y in 0..height {
        for x in 0..width {
            spawn_unset_cell(&mut commands, (x, y), (w, h), &cell_meshes, &cell_colors);
        }
    }
    for wnd in wnds.iter() {
        // TODO: Remove hard-coded width/height
        rescale_board(
            width,
            height,
            4,
            wnd.width(),
            wnd.height(),
            &mut camera_query,
        );
    }
}

fn spawn_unset_cell(
    commands: &mut Commands,
    pos: (usize, usize),
    size: (f32, f32),
    cell_meshes: &CellMeshes,
    cell_colors: &CellColors,
) {
    let cell = commands.spawn().id();

    let big_transform = spawn_cell_common(
        commands,
        cell,
        cell_meshes,
        (cell_colors.alpha1.clone(), cell_colors.alpha0.clone()),
        pos,
        size,
        (true, false),
    );

    commands
        .entity(cell)
        .insert_bundle(ColorMesh2dBundle {
            mesh: cell_meshes.big_hexagon.clone().into(),
            material: cell_colors.alpha0.clone(),
            transform: big_transform,
            ..default()
        })
        .insert(UnsetCell);
}

pub fn spawn_cell_common(
    commands: &mut Commands,
    cell: Entity,
    cell_meshes: &CellMeshes,
    (c1, c2): (Handle<ColorMaterial>, Handle<ColorMaterial>),
    (x, y): (usize, usize),
    (w, h): (f32, f32),
    mouse: (bool, bool),
) -> Transform {
    let tx = x as f32 * RADIUS * 1.56 - w;
    let ty = y as f32 * RADIUS * -1.8
        + match x % 2 {
            0 => 0.,
            _ => RADIUS * 0.9,
        }
        + h;

    let mut big_transform = Transform::from_translation(Vec3::new(tx, ty, Z_INDEX_CELL_BACK));
    big_transform.rotate_z(f32::to_radians(90.0));

    let b1 = ColorMesh2dBundle {
        mesh: cell_meshes.medium_hexagon.clone().into(),
        material: c1,
        transform: OUTER_TRANSFORM,
        ..default()
    };
    let b2 = ColorMesh2dBundle {
        mesh: cell_meshes.small_hexagon.clone().into(),
        material: c2,
        transform: INNER_TRANSFORM,
        ..default()
    };

    // do the same for the child
    let child1 = commands.spawn_bundle(b1).insert(CellOuter).id();
    let child2 = commands.spawn_bundle(b2).insert(CellInner).id();

    commands.entity(cell).push_children(&[child1, child2]);

    make_cell_interactable(commands, cell, mouse);

    let cell_component = Cell {
        x,
        y,
        entity: cell,
        outer_hexagon: child1,
        inner_hexagon: child2,
        orig: big_transform,
        hovering: false,
    };
    // TODO: Rethink Cell type
    commands.entity(cell).insert(cell_component);

    big_transform
}
