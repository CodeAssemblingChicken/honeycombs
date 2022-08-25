use crate::{
    components::Cell,
    resources::{CellColors, SfxHover},
};
use bevy::{
    audio::Audio,
    prelude::{Camera, Commands, EventReader, Handle, Query, Res, ResMut, Transform, With},
    sprite::ColorMaterial,
    window::WindowResized,
};
use interactable::{
    click::MouseLeftReleasedEvent,
    hover::{MouseEnterEvent, MouseExitEvent, MouseOverEvent},
};

pub fn mouse_click_cell(
    mut commands: Commands,
    mut cell_query: Query<&mut Cell>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: ResMut<CellColors>,
    mut ev_mouse_left_click: EventReader<MouseLeftReleasedEvent>,
) {
    for ev in ev_mouse_left_click.iter() {
        if let Ok(mut cell) = cell_query.get_mut(ev.0) {
            cell.click(
                &mut commands,
                cell_colors.blue_light.id,
                cell_colors.blue_medium.id,
                &mut color_query,
            );
        }
    }
}

/// Calls hover on a cell that is entered by the mouse
pub fn mouse_enter_cell(
    mut commands: Commands,
    mut cell_query: Query<&mut Cell>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: ResMut<CellColors>,
    mut ev_mouse_enter: EventReader<MouseEnterEvent>,
    audio: Res<Audio>,
    clip: Res<SfxHover>,
) {
    for ev in ev_mouse_enter.iter() {
        if let Ok(mut cell) = cell_query.get_mut(ev.0) {
            println!("findet");
            cell.hover(
                &mut commands,
                cell_colors.blue_medium.id,
                cell_colors.blue_dark.id,
                &mut color_query,
            );
        }
    }
}

/// Calls unhover on a cell that is exited by the mouse
pub fn mouse_exit_cell(
    mut commands: Commands,
    mut cell_query: Query<&mut Cell>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: ResMut<CellColors>,
    mut ev_mouse_exit: EventReader<MouseExitEvent>,
) {
    for ev in ev_mouse_exit.iter() {
        if let Ok(mut cell) = cell_query.get_mut(ev.0) {
            cell.unhover(
                &mut commands,
                cell_colors.blue_light.id,
                cell_colors.blue_medium.id,
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
    for ev in ev_mouse_over.iter() {
        // println!("{} is hovered.", ev.0.id());
    }
}

/// On resizing the window, the board is resized too
/// i.e. the camera zoom (scale) is recalculated
pub fn window_resize_system(
    mut ev_window_resize: EventReader<WindowResized>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    // board: Query<&Board>,
) {
    for ev in ev_window_resize.iter() {
        // let w = ((b.width + 4) as f32 * RADIUS * 1.56) / ev.width;
        // let h = ((b.height + 4) as f32 * RADIUS * 1.8) / ev.height;
        // let s = w.max(h);
        // for mut t in camera_query.iter_mut() {
        //     t.scale = Vec3::new(s, s, 1.0);
        // }
    }
}
