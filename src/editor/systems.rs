use crate::{
    components::{Cell, CellType, HintType},
    functions::{rescale_board, spawn_cell_text},
    resources::{CellColors, TextSettings},
};
use bevy::{
    hierarchy::BuildChildren,
    prelude::{Camera, Commands, EventReader, Handle, Query, Res, Transform, With},
    sprite::ColorMaterial,
    window::WindowResized,
};
use interactable::{
    click::{ClickType, MouseLeftClickEvent, MouseMiddleClickEvent, MouseRightClickEvent},
    hover::{MouseEnterEvent, MouseExitEvent},
};

use super::components::{EditorCell, EmptyCell, NumberCell, UnsetCell};

pub fn mouse_click_unset_cell(
    mut commands: Commands,
    mut cell_query: Query<(&mut EditorCell, &mut Cell), With<UnsetCell>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: Res<CellColors>,
    text_settings: Res<TextSettings>,
    mut ev_mouse_left_click: EventReader<MouseLeftClickEvent>,
    mut ev_mouse_right_click: EventReader<MouseRightClickEvent>,
) {
    for ev in ev_mouse_left_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok((mut ec, mut cell)) = cell_query.get_mut(ev.entity) {
            ec.cell_type = Some(CellType::EmptyCell);
            commands
                .entity(ev.entity)
                .remove::<UnsetCell>()
                .insert(EmptyCell);
            cell.click(
                &mut commands,
                Some(cell_colors.white.clone()),
                cell_colors.blue_light.clone(),
                cell_colors.blue_medium.clone(),
                &mut color_query,
            );
        }
    }
    for ev in ev_mouse_right_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok((mut ec, mut cell)) = cell_query.get_mut(ev.entity) {
            ec.cell_type = Some(CellType::NumberCell(HintType::None));
            let count = 1;
            let text_entity = spawn_cell_text(&mut commands, &format!("{}", count), &text_settings);
            commands.entity(ev.entity).add_child(text_entity);
            commands
                .entity(ev.entity)
                .remove::<UnsetCell>()
                .insert(NumberCell {
                    count,
                    label: text_entity,
                    hint_type: HintType::None,
                });
            cell.click(
                &mut commands,
                Some(cell_colors.white.clone()),
                cell_colors.gray_light.clone(),
                cell_colors.gray_medium.clone(),
                &mut color_query,
            );
            ec.hover(&mut cell, &mut commands, &mut color_query, &cell_colors);
        }
    }
}

pub fn mouse_click_empty_cell(
    mut commands: Commands,
    mut cell_query: Query<(&mut EditorCell, &mut Cell), With<EmptyCell>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: Res<CellColors>,
    text_settings: Res<TextSettings>,
    mut ev_mouse_left_click: EventReader<MouseLeftClickEvent>,
    mut ev_mouse_right_click: EventReader<MouseRightClickEvent>,
    mut ev_mouse_middle_click: EventReader<MouseMiddleClickEvent>,
) {
    for ev in ev_mouse_left_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok((mut ec, mut cell)) = cell_query.get_mut(ev.entity) {
            ec.hidden = !ec.hidden;
            let (c1, c2) = if ec.hidden {
                (
                    cell_colors.yellow_light.clone(),
                    cell_colors.yellow_medium.clone(),
                )
            } else {
                (
                    cell_colors.blue_light.clone(),
                    cell_colors.blue_medium.clone(),
                )
            };
            cell.click(
                &mut commands,
                Some(cell_colors.white.clone()),
                c1,
                c2,
                &mut color_query,
            );
        }
    }
    for ev in ev_mouse_right_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok((mut ec, mut cell)) = cell_query.get_mut(ev.entity) {
            ec.cell_type = Some(CellType::NumberCell(HintType::None));
            ec.hidden = false;
            let count = 1;
            let text_entity = spawn_cell_text(&mut commands, &format!("{}", count), &text_settings);
            commands.entity(ev.entity).add_child(text_entity);
            commands
                .entity(ev.entity)
                .remove::<EmptyCell>()
                .insert(NumberCell {
                    count,
                    label: text_entity,
                    hint_type: HintType::None,
                });
            cell.click(
                &mut commands,
                Some(cell_colors.white.clone()),
                cell_colors.gray_light.clone(),
                cell_colors.gray_medium.clone(),
                &mut color_query,
            );
            ec.hover(&mut cell, &mut commands, &mut color_query, &cell_colors);
        }
    }
    for ev in ev_mouse_middle_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok((mut ec, mut cell)) = cell_query.get_mut(ev.entity) {
            ec.cell_type = None;
            ec.hidden = false;
            commands
                .entity(ev.entity)
                .remove::<EmptyCell>()
                .insert(UnsetCell);
            cell.click(
                &mut commands,
                Some(cell_colors.alpha0.clone()),
                cell_colors.alpha0.clone(),
                cell_colors.alpha1.clone(),
                &mut color_query,
            );
        }
    }
}

pub fn mouse_click_number_cell(
    mut commands: Commands,
    mut cell_query: Query<(&mut EditorCell, &mut Cell, &NumberCell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: Res<CellColors>,
    mut ev_mouse_left_click: EventReader<MouseLeftClickEvent>,
    mut ev_mouse_right_click: EventReader<MouseRightClickEvent>,
    mut ev_mouse_middle_click: EventReader<MouseMiddleClickEvent>,
) {
    for ev in ev_mouse_left_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok((mut ec, mut cell, nc)) = cell_query.get_mut(ev.entity) {
            ec.cell_type = Some(CellType::EmptyCell);
            ec.hidden = false;
            commands.entity(nc.label).despawn();
            commands
                .entity(ev.entity)
                .remove::<NumberCell>()
                .insert(EmptyCell);
            cell.click(
                &mut commands,
                Some(cell_colors.white.clone()),
                cell_colors.blue_light.clone(),
                cell_colors.blue_medium.clone(),
                &mut color_query,
            );
        }
    }
    for ev in ev_mouse_right_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok((mut ec, mut cell, _nc)) = cell_query.get_mut(ev.entity) {
            ec.hidden = !ec.hidden;
            let (c1, c2) = if ec.hidden {
                (
                    cell_colors.yellow_light.clone(),
                    cell_colors.yellow_medium.clone(),
                )
            } else {
                (
                    cell_colors.gray_light.clone(),
                    cell_colors.gray_medium.clone(),
                )
            };
            cell.click(
                &mut commands,
                Some(cell_colors.white.clone()),
                c1,
                c2,
                &mut color_query,
            );
        }
    }
    for ev in ev_mouse_middle_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok((mut ec, mut cell, nc)) = cell_query.get_mut(ev.entity) {
            ec.cell_type = None;
            ec.hidden = false;
            commands.entity(nc.label).despawn();
            commands
                .entity(ev.entity)
                .remove::<NumberCell>()
                .insert(UnsetCell);
            cell.click(
                &mut commands,
                Some(cell_colors.alpha0.clone()),
                cell_colors.alpha0.clone(),
                cell_colors.alpha1.clone(),
                &mut color_query,
            );
        }
    }
}

/// Calls hover on a cell that is entered by the mouse
pub fn mouse_enter_cell(
    mut commands: Commands,
    mut cell_query: Query<(&EditorCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: Res<CellColors>,
    mut ev_mouse_enter: EventReader<MouseEnterEvent>,
    // audio: Res<Audio>,
    // clip: Res<SfxHover>,
) {
    for ev in ev_mouse_enter.iter() {
        if let Ok((ec, mut cell)) = cell_query.get_mut(ev.0) {
            ec.hover(&mut cell, &mut commands, &mut color_query, &cell_colors);
        }
    }
}

/// Calls unhover on a cell that is exited by the mouse
pub fn mouse_exit_cell(
    mut commands: Commands,
    mut cell_query: Query<(&EditorCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: Res<CellColors>,
    mut ev_mouse_exit: EventReader<MouseExitEvent>,
) {
    for ev in ev_mouse_exit.iter() {
        if let Ok((ec, mut cell)) = cell_query.get_mut(ev.0) {
            ec.unhover(&mut cell, &mut commands, &mut color_query, &cell_colors);
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
