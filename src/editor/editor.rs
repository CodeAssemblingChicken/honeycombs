use bevy::{
    math::Vec3,
    prelude::{Camera, Commands, Entity, Handle, Query, Res, Transform, With},
    sprite::ColorMaterial,
    window::Windows,
};

use crate::{
    components::Cell,
    constants::{RADIUS, Z_INDEX_CELL_BACK},
    editor::components::UnsetCell,
    functions::{calc_translation, make_cell_interactable, rescale_board, spawn_cell},
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

    for y in 0..height as i32 {
        for x in 0..width as i32 {
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
    pos: (i32, i32),
    size: (f32, f32),
    cell_meshes: &CellMeshes,
    cell_colors: &CellColors,
) {
    let cell = commands.spawn().id();
    spawn_cell_common(
        commands,
        cell,
        cell_meshes,
        (
            cell_colors.alpha0.clone(),
            cell_colors.alpha1.clone(),
            cell_colors.alpha0.clone(),
        ),
        pos,
        size,
        (true, false),
    );
    commands.entity(cell).insert(UnsetCell);
}

pub fn spawn_cell_common(
    commands: &mut Commands,
    cell: Entity,
    cell_meshes: &CellMeshes,
    colors: (
        Handle<ColorMaterial>,
        Handle<ColorMaterial>,
        Handle<ColorMaterial>,
    ),
    (x, y): (i32, i32),
    (w, h): (f32, f32),
    mouse: (bool, bool),
) {
    let (tx, ty) = calc_translation(x, y, w, h);
    let mut big_transform = Transform::from_translation(Vec3::new(tx, ty, Z_INDEX_CELL_BACK));
    big_transform.rotate_z(f32::to_radians(90.0));

    let (child1, child2) = spawn_cell(commands, cell, cell_meshes, colors, big_transform);

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
}
