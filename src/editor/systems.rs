use crate::{components::Cell, functions::rescale_board, resources::CellColors};
use bevy::{
    hierarchy::DespawnRecursiveExt,
    prelude::{Camera, Commands, EventReader, Handle, Query, Res, Transform, With},
    sprite::ColorMaterial,
    window::WindowResized,
};
use interactable::{
    click::MouseLeftReleasedEvent,
    hover::{MouseEnterEvent, MouseExitEvent},
};

use super::components::UnsetCell;

pub fn mouse_click_unset_cell(
    mut commands: Commands,
    mut cell_query: Query<(&mut Cell, &Transform), With<UnsetCell>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: Res<CellColors>,
    mut ev_mouse_left_click: EventReader<MouseLeftReleasedEvent>,
) {
    for ev in ev_mouse_left_click.iter() {
        if let Ok((mut cell, t)) = cell_query.get_mut(ev.0) {
            commands.entity(ev.0).despawn_recursive();
        }
    }
}

/// Calls hover on a cell that is entered by the mouse
pub fn mouse_enter_unset_cell(
    mut commands: Commands,
    mut cell_query: Query<&mut Cell, With<UnsetCell>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: Res<CellColors>,
    mut ev_mouse_enter: EventReader<MouseEnterEvent>,
    // audio: Res<Audio>,
    // clip: Res<SfxHover>,
) {
    for ev in ev_mouse_enter.iter() {
        if let Ok(mut cell) = cell_query.get_mut(ev.0) {
            cell.hover(
                &mut commands,
                cell_colors.alpha0.clone(),
                cell_colors.alpha2.clone(),
                &mut color_query,
            );
        }
    }
}

/// Calls unhover on a cell that is exited by the mouse
pub fn mouse_exit_unset_cell(
    mut commands: Commands,
    mut cell_query: Query<&mut Cell, With<UnsetCell>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: Res<CellColors>,
    mut ev_mouse_exit: EventReader<MouseExitEvent>,
) {
    for ev in ev_mouse_exit.iter() {
        if let Ok(mut cell) = cell_query.get_mut(ev.0) {
            cell.unhover(
                &mut commands,
                cell_colors.alpha0.clone(),
                cell_colors.alpha1.clone(),
                &mut color_query,
            );
        }
    }
}

/// On resizing the window, the board is resized too
/// i.e. the camera zoom (scale) is recalculated
pub fn window_resize_system(
    mut ev_window_resize: EventReader<WindowResized>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    // board: Res<Board>,
) {
    for ev in ev_window_resize.iter() {
        rescale_board(15, 10, 3, ev.width, ev.height, &mut camera_query);
    }
}
