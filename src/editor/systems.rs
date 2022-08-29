use super::components::{Board, CellUpdateEvent, EditorCell, EmptyCell, NumberCell, UnsetCell};
use crate::{
    board_functions::{count_empty_cells, empty_connected, get_neighbours},
    components::{Cell, CellType, HintType},
    functions::{rescale_board, spawn_cell_text},
    resources::{CellColors, TextSettings},
};
use bevy::{
    hierarchy::BuildChildren,
    prelude::{
        Camera, Color, Commands, EventReader, EventWriter, Handle, Query, Res, ResMut, Transform,
        With,
    },
    sprite::ColorMaterial,
    text::Text,
    window::WindowResized,
};
use interactable::{
    click::{ClickType, MouseLeftClickEvent, MouseMiddleClickEvent, MouseRightClickEvent},
    hover::{MouseEnterEvent, MouseExitEvent},
};

pub fn mouse_click_unset_cell(
    mut commands: Commands,
    mut cell_query: Query<(&mut EditorCell, &mut Cell), With<UnsetCell>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    (cell_colors, text_settings): (Res<CellColors>, Res<TextSettings>),
    mut board: ResMut<Board>,
    (mut ev_mouse_left_click, mut ev_mouse_right_click): (
        EventReader<MouseLeftClickEvent>,
        EventReader<MouseRightClickEvent>,
    ),
    mut ev_cell_update: EventWriter<CellUpdateEvent>,
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
            board.cells[cell.y as usize][cell.x as usize] = Some(CellType::EmptyCell);
            ev_cell_update.send(CellUpdateEvent);
        }
    }
    for ev in ev_mouse_right_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok((mut ec, mut cell)) = cell_query.get_mut(ev.entity) {
            ec.cell_type = Some(CellType::NumberCell(HintType::None));
            let count = 0;
            let text_entity = spawn_cell_text(&mut commands, &format!("{}", count), &text_settings);
            commands.entity(ev.entity).add_child(text_entity);
            commands
                .entity(ev.entity)
                .remove::<UnsetCell>()
                .insert(NumberCell {
                    count,
                    label: text_entity,
                    special_hint: false,
                });
            cell.click(
                &mut commands,
                Some(cell_colors.white.clone()),
                cell_colors.gray_light.clone(),
                cell_colors.gray_medium.clone(),
                &mut color_query,
            );
            board.cells[cell.y as usize][cell.x as usize] =
                Some(CellType::NumberCell(HintType::None));
            ev_cell_update.send(CellUpdateEvent);
        }
    }
    ev_mouse_left_click.clear();
    ev_mouse_right_click.clear();
}

pub fn mouse_click_empty_cell(
    mut commands: Commands,
    mut cell_query: Query<(&mut EditorCell, &mut Cell), With<EmptyCell>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: Res<CellColors>,
    mut board: ResMut<Board>,
    (mut ev_mouse_left_click, mut ev_mouse_middle_click): (
        EventReader<MouseLeftClickEvent>,
        EventReader<MouseMiddleClickEvent>,
    ),
    mut ev_cell_update: EventWriter<CellUpdateEvent>,
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
            ev_cell_update.send(CellUpdateEvent);
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
            board.cells[cell.y as usize][cell.x as usize] = None;
            ev_cell_update.send(CellUpdateEvent);
        }
    }
    ev_mouse_left_click.clear();
    ev_mouse_middle_click.clear();
}

pub fn mouse_click_number_cell(
    mut commands: Commands,
    mut cell_query: Query<(&mut EditorCell, &mut Cell, &mut NumberCell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: Res<CellColors>,
    mut board: ResMut<Board>,
    (mut ev_mouse_left_click, mut ev_mouse_right_click, mut ev_mouse_middle_click): (
        EventReader<MouseLeftClickEvent>,
        EventReader<MouseRightClickEvent>,
        EventReader<MouseMiddleClickEvent>,
    ),
    mut ev_cell_update: EventWriter<CellUpdateEvent>,
) {
    for ev in ev_mouse_left_click
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
            ev_cell_update.send(CellUpdateEvent);
        }
    }
    for ev in ev_mouse_right_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok((_ec, _cell, mut nc)) = cell_query.get_mut(ev.entity) {
            nc.special_hint = !nc.special_hint;
            ev_cell_update.send(CellUpdateEvent);
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
            board.cells[cell.y as usize][cell.x as usize] = None;
            ev_cell_update.send(CellUpdateEvent);
        }
    }
    ev_mouse_left_click.clear();
    ev_mouse_right_click.clear();
    ev_mouse_middle_click.clear();
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

pub fn cell_update_system(
    mut cell_query: Query<(&mut Cell, &mut NumberCell)>,
    mut text_query: Query<&mut Text>,
    text_settings: Res<TextSettings>,
    board: Res<Board>,
    mut ev_cell_update: EventReader<CellUpdateEvent>,
) {
    for _ev in ev_cell_update.iter() {
        for (cell, mut nc) in cell_query.iter_mut() {
            let neighbours =
                get_neighbours(cell.x, cell.y, &board.cells, board.width, board.height);
            let count = count_empty_cells(&neighbours);
            nc.count = count;
            let mut ts = text_settings.clone();
            if nc.special_hint {
                if empty_connected(&neighbours, count, true) {
                    ts.style.color = Color::GREEN;
                } else {
                    ts.style.color = Color::RED;
                }
            }
            *text_query.get_mut(nc.label).unwrap() =
                Text::from_section(&format!("{}", count), ts.style)
                    .with_alignment(text_settings.alignment);
        }
    }
}

/// On resizing the window, the board is resized too
/// i.e. the camera zoom (scale) is recalculated
pub fn window_resize_system(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    board: Res<Board>,
    mut ev_window_resize: EventReader<WindowResized>,
    // board: Res<Board>,
) {
    for ev in ev_window_resize.iter() {
        rescale_board(
            board.width,
            board.height,
            3,
            ev.width,
            ev.height,
            &mut camera_query,
        );
    }
}
