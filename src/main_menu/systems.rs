use super::components::{LevelSelectionCell, OptionCell};
use crate::{
    components::Cell,
    functions::rescale_board,
    resources::{CellColors, LevelFile},
    states::AppState,
};
use bevy::{
    prelude::{
        Camera, Commands, EventReader, Handle, Query, Res, ResMut, State, Transform, With, Without,
    },
    sprite::ColorMaterial,
    window::WindowResized,
};
use interactable::{
    click::{ClickType, MouseLeftClickEvent},
    hover::{MouseEnterEvent, MouseExitEvent, MouseOverEvent},
};

pub fn mouse_click_cell(
    mut commands: Commands,
    mut level_cell_query: Query<(&LevelSelectionCell, &mut Cell)>,
    mut editor_cell_query: Query<(&OptionCell, &mut Cell), Without<LevelSelectionCell>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: Res<CellColors>,
    (mut app_state, mut level_file): (ResMut<State<AppState>>, ResMut<LevelFile>),
    mut ev_mouse_left_click: EventReader<MouseLeftClickEvent>,
) {
    for ev in ev_mouse_left_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok((lsc, mut cell)) = level_cell_query.get_mut(ev.entity) {
            lsc.click(
                &mut cell,
                &mut commands,
                &mut color_query,
                &cell_colors,
                &mut app_state,
                &mut level_file,
            );
        }
        if let Ok((ec, mut cell)) = editor_cell_query.get_mut(ev.entity) {
            ec.click(
                &mut cell,
                &mut commands,
                &mut color_query,
                &cell_colors,
                &mut app_state,
            );
        }
    }
}

/// Calls hover on a cell that is entered by the mouse
pub fn mouse_enter_cell(
    mut commands: Commands,
    mut level_cell_query: Query<(&LevelSelectionCell, &mut Cell)>,
    mut editor_cell_query: Query<&mut Cell, (With<OptionCell>, Without<LevelSelectionCell>)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: Res<CellColors>,
    mut ev_mouse_enter: EventReader<MouseEnterEvent>,
    // audio: Res<Audio>,
    // clip: Res<SfxHover>,
) {
    for ev in ev_mouse_enter.iter() {
        if let Ok((lsc, mut cell)) = level_cell_query.get_mut(ev.0) {
            lsc.hover(&mut cell, &mut commands, &mut color_query, &cell_colors);
        }
        if let Ok(mut cell) = editor_cell_query.get_mut(ev.0) {
            cell.hover(
                &mut commands,
                None,
                cell_colors.gray_medium.clone(),
                cell_colors.gray_dark.clone(),
                &mut color_query,
            );
        }
    }
}

/// Calls unhover on a cell that is exited by the mouse
pub fn mouse_exit_cell(
    mut commands: Commands,
    mut level_cell_query: Query<(&LevelSelectionCell, &mut Cell)>,
    mut editor_cell_query: Query<&mut Cell, (With<OptionCell>, Without<LevelSelectionCell>)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: Res<CellColors>,
    mut ev_mouse_exit: EventReader<MouseExitEvent>,
) {
    for ev in ev_mouse_exit.iter() {
        if let Ok((lsc, mut cell)) = level_cell_query.get_mut(ev.0) {
            lsc.unhover(&mut cell, &mut commands, &mut color_query, &cell_colors);
        }
        if let Ok(mut cell) = editor_cell_query.get_mut(ev.0) {
            cell.unhover(
                &mut commands,
                None,
                cell_colors.gray_light.clone(),
                cell_colors.gray_medium.clone(),
                &mut color_query,
            );
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
