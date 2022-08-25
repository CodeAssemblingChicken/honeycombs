use super::{
    board::Board,
    components::{EmptyCell, GameCell, NumberCell},
    functions::rescale_board,
};
use crate::{
    components::Cell,
    resources::{CellColors, SfxHover},
};
use bevy::{
    audio::{Audio, PlaybackSettings},
    prelude::{
        Camera, ColorMaterial, Commands, EventReader, Handle, Query, Res, ResMut, Transform, With,
        Without,
    },
    window::WindowResized,
};
use interactable::{
    click::{MouseLeftReleasedEvent, MouseRightReleasedEvent},
    hover::{MouseEnterEvent, MouseExitEvent, MouseOverEvent},
};

/// Calls uncover on a cell that is clicked by the mouse
pub fn mouse_click_cell(
    mut commands: Commands,
    mut number_cell_query: Query<(&GameCell, &mut Cell, &NumberCell), Without<EmptyCell>>,
    mut empty_cell_query: Query<(&GameCell, &mut Cell), With<EmptyCell>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: ResMut<CellColors>,
    mut ev_mouse_left_click: EventReader<MouseLeftReleasedEvent>,
    mut ev_mouse_right_click: EventReader<MouseRightReleasedEvent>,
    mut board: ResMut<Board>,
) {
    for ev in ev_mouse_left_click.iter() {
        if let Ok((lc, cell, _nc)) = number_cell_query.get(ev.0) {
            lc.uncover_fail(cell, &mut commands);
        }
        if let Ok((lc, mut cell)) = empty_cell_query.get_mut(ev.0) {
            lc.uncover(
                &mut cell,
                &mut commands,
                &mut color_query,
                cell_colors.as_ref(),
                None,
                &mut board,
            );
        }
    }
    for ev in ev_mouse_right_click.iter() {
        if let Ok((lc, mut cell, nc)) = number_cell_query.get_mut(ev.0) {
            lc.uncover(
                &mut cell,
                &mut commands,
                &mut color_query,
                cell_colors.as_ref(),
                Some(nc),
                &mut board,
            );
        }
        if let Ok((lc, cell)) = empty_cell_query.get(ev.0) {
            lc.uncover_fail(cell, &mut commands);
        }
    }
}

/// Calls hover on a cell that is entered by the mouse
pub fn mouse_enter_cell(
    mut commands: Commands,
    mut cell_query: Query<(&GameCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: ResMut<CellColors>,
    mut ev_mouse_enter: EventReader<MouseEnterEvent>,
    audio: Res<Audio>,
    clip: Res<SfxHover>,
) {
    for ev in ev_mouse_enter.iter() {
        if let Ok((lc, mut cell)) = cell_query.get_mut(ev.0) {
            audio.play_with_settings(clip.0.clone(), PlaybackSettings::ONCE.with_volume(0.05));
            lc.hover(&mut cell, &mut commands, &mut color_query, &cell_colors);
        }
    }
}

/// Calls unhover on a cell that is exited by the mouse
pub fn mouse_exit_cell(
    mut commands: Commands,
    mut cell_query: Query<(&GameCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: ResMut<CellColors>,
    mut ev_mouse_exit: EventReader<MouseExitEvent>,
) {
    for ev in ev_mouse_exit.iter() {
        if let Ok((lc, mut cell)) = cell_query.get_mut(ev.0) {
            lc.unhover(&mut cell, &mut commands, &mut color_query, &cell_colors);
        }
    }
}

// TODO: Not used
/// Could call a function on the currently hovered cell, but doesn't right now
#[allow(unused_mut, unused_variables)]
pub fn mouse_over_cell(
    mut commands: Commands,
    cell_query: Query<&Transform, With<Cell>>,
    mut ev_mouse_over: EventReader<MouseOverEvent>,
) {
    for ev in ev_mouse_over.iter() {}
}

/// On resizing the window, the board is resized too
/// i.e. the camera zoom (scale) is recalculated
pub fn window_resize_system(
    mut ev_window_resize: EventReader<WindowResized>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    board: Res<Board>,
) {
    for ev in ev_window_resize.iter() {
        rescale_board(&board, ev.width, ev.height, &mut camera_query);
    }
}

pub fn check_solved(board: Res<Board>) {
    if board.is_changed() && board.remaining == 0 {}
}
