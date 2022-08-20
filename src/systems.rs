use crate::components::{Cell, CellColors, EmptyCell, NumberCell, SfxHover, TextSettings};
use bevy::{
    audio::{Audio, PlaybackSettings},
    prelude::{
        ColorMaterial, Commands, EventReader, Handle, Query, Res, ResMut, Transform, With, Without,
    },
    window::WindowResized,
};
use interactable::{
    click::{MouseLeftReleasedEvent, MouseRightReleasedEvent},
    hover::{MouseEnterEvent, MouseExitEvent, MouseOverEvent},
};

pub fn mouse_click_cell(
    mut commands: Commands,
    mut number_cell_query: Query<(&mut Cell, &NumberCell), Without<EmptyCell>>,
    mut empty_cell_query: Query<&mut Cell, With<EmptyCell>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: ResMut<CellColors>,
    text_settings: Res<TextSettings>,
    mut ev_mouse_left_click: EventReader<MouseLeftReleasedEvent>,
    mut ev_mouse_right_click: EventReader<MouseRightReleasedEvent>,
) {
    for ev in ev_mouse_left_click.iter() {
        if let Ok((cell, _nc)) = number_cell_query.get(ev.0) {
            cell.uncover_fail(&mut commands);
        }
        if let Ok(mut cell) = empty_cell_query.get_mut(ev.0) {
            cell.uncover_empty(&mut commands, &mut color_query, cell_colors.as_ref());
        }
    }
    for ev in ev_mouse_right_click.iter() {
        if let Ok((mut cell, nc)) = number_cell_query.get_mut(ev.0) {
            cell.uncover_number(
                &mut commands,
                &mut color_query,
                cell_colors.as_ref(),
                nc,
                text_settings.as_ref(),
            );
        }
        if let Ok(cell) = empty_cell_query.get(ev.0) {
            cell.uncover_fail(&mut commands);
        }
    }
}

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
            audio.play_with_settings(clip.0.clone(), PlaybackSettings::ONCE.with_volume(0.05));
            cell.hover(&mut commands, &mut color_query, &cell_colors);
        }
    }
}
pub fn mouse_exit_cell(
    mut commands: Commands,
    mut cell_query: Query<&mut Cell>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: ResMut<CellColors>,
    mut ev_mouse_exit: EventReader<MouseExitEvent>,
) {
    for ev in ev_mouse_exit.iter() {
        if let Ok(mut cell) = cell_query.get_mut(ev.0) {
            cell.unhover(&mut commands, &mut color_query, &cell_colors);
        }
    }
}

#[allow(unused_mut, unused_variables)]
pub fn mouse_over_cell(
    mut commands: Commands,
    cell_query: Query<&Transform, With<Cell>>,
    mut ev_mouse_over: EventReader<MouseOverEvent>,
) {
    for ev in ev_mouse_over.iter() {
        // println!("{} is hovered.", ev.0.id());
    }
}

pub fn window_resize_system(mut ev_window_resize: EventReader<WindowResized>) {
    for ev in ev_window_resize.iter() {
        println!("{}×{}", ev.width, ev.height);
    }
}
