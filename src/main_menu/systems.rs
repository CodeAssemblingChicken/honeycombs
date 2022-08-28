use super::components::LevelSelectionCell;
use crate::{
    components::Cell, functions::rescale_board, level::resources::LevelFile, resources::CellColors,
    states::AppState,
};
use bevy::{
    prelude::{Camera, Commands, EventReader, Handle, Query, Res, ResMut, State, Transform, With},
    sprite::ColorMaterial,
    window::WindowResized,
};
use interactable::{
    click::MouseLeftReleasedEvent,
    hover::{MouseEnterEvent, MouseExitEvent, MouseOverEvent},
};

pub fn mouse_click_cell(
    mut commands: Commands,
    mut cell_query: Query<(&LevelSelectionCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: Res<CellColors>,
    mut ev_mouse_left_click: EventReader<MouseLeftReleasedEvent>,
    mut app_state: ResMut<State<AppState>>,
    mut level_file: ResMut<LevelFile>,
) {
    for ev in ev_mouse_left_click.iter() {
        if let Ok((lsc, mut cell)) = cell_query.get_mut(ev.0) {
            lsc.click(
                &mut cell,
                &mut commands,
                &mut color_query,
                &cell_colors,
                &mut app_state,
                &mut level_file,
            );
        }
    }
}

/// Calls hover on a cell that is entered by the mouse
pub fn mouse_enter_cell(
    mut commands: Commands,
    mut cell_query: Query<(&LevelSelectionCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: Res<CellColors>,
    mut ev_mouse_enter: EventReader<MouseEnterEvent>,
    // audio: Res<Audio>,
    // clip: Res<SfxHover>,
) {
    for ev in ev_mouse_enter.iter() {
        if let Ok((lsc, mut cell)) = cell_query.get_mut(ev.0) {
            lsc.hover(&mut cell, &mut commands, &mut color_query, &cell_colors);
        }
    }
}

/// Calls unhover on a cell that is exited by the mouse
pub fn mouse_exit_cell(
    mut commands: Commands,
    mut cell_query: Query<(&LevelSelectionCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: Res<CellColors>,
    mut ev_mouse_exit: EventReader<MouseExitEvent>,
) {
    for ev in ev_mouse_exit.iter() {
        if let Ok((lsc, mut cell)) = cell_query.get_mut(ev.0) {
            lsc.unhover(&mut cell, &mut commands, &mut color_query, &cell_colors);
        }
    }
}

// TODO: Not used
/// Could call a function on the currently hovered cell, but doesn't right now
#[allow(unused_mut, unused_variables)]
pub fn mouse_over_cell(
    mut commands: Commands,
    mut cell_query: Query<&mut Cell>,
    mut ev_mouse_over: EventReader<MouseOverEvent>,
) {
    for ev in ev_mouse_over.iter() {}
}

/// On resizing the window, the board is resized too
/// i.e. the camera zoom (scale) is recalculated
pub fn window_resize_system(
    mut ev_window_resize: EventReader<WindowResized>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    for ev in ev_window_resize.iter() {
        // TODO: Remove hard-coded width/height
        rescale_board(11, 11, 1, ev.width, ev.height, &mut camera_query);
    }
}
