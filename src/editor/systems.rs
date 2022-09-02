use super::{
    board::Board,
    components::{CellUpdateEvent, EditorCell, EmptyCell, NumberCell, UnsetCell},
    functions::{set_empty_cell, set_number_cell, unset_cell},
};
use crate::{
    board_functions::{count_empty_cells, empty_connected, get_neighbours},
    components::{BoardConfig, Cell, CellType, HintType},
    functions::rescale_board,
    parser::board_to_string,
    resources::{CellColors, TextSettings},
};
use bevy::{
    input::Input,
    prelude::{
        Camera, Color, Commands, EventReader, EventWriter, Handle, KeyCode, Query, Res, ResMut,
        Transform, With,
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
    cell_colors: Res<CellColors>,
    mut board: ResMut<Board>,
    (mut ev_mouse_left_click, mut ev_mouse_right_click): (
        EventReader<MouseLeftClickEvent>,
        EventReader<MouseRightClickEvent>,
    ),
    mut ev_cell_update: EventWriter<CellUpdateEvent>,
) {
    for ev in ev_mouse_left_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Pressed)
    {
        if let Ok((mut ec, mut cell)) = cell_query.get_mut(ev.entity) {
            set_empty_cell(
                &mut commands,
                ev.entity,
                (&mut cell, &mut ec),
                &mut color_query,
                &cell_colors,
                &mut board,
                &mut ev_cell_update,
            )
        }
    }
    for ev in ev_mouse_right_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Pressed)
    {
        if let Ok((mut ec, mut cell)) = cell_query.get_mut(ev.entity) {
            set_number_cell(
                &mut commands,
                ev.entity,
                (&mut cell, &mut ec),
                &mut color_query,
                &cell_colors,
                &mut board,
                &mut ev_cell_update,
            );
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
        .filter(|ev| ev.click_type == ClickType::Just)
    {
        if let Ok((mut ec, mut cell)) = cell_query.get_mut(ev.entity) {
            ec.toggle_hidden(
                &mut cell,
                &mut commands,
                &mut color_query,
                &cell_colors,
                &mut board,
                &mut ev_cell_update,
            );
        }
    }
    for ev in ev_mouse_middle_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Pressed)
    {
        if let Ok((mut ec, mut cell)) = cell_query.get_mut(ev.entity) {
            commands.entity(ev.entity).remove::<EmptyCell>();
            unset_cell(
                &mut commands,
                ev.entity,
                (&mut cell, &mut ec),
                &mut color_query,
                &cell_colors,
                &mut board,
                &mut ev_cell_update,
            );
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
        .filter(|ev| ev.click_type == ClickType::Just)
    {
        if let Ok((mut ec, mut cell, _nc)) = cell_query.get_mut(ev.entity) {
            ec.toggle_hidden(
                &mut cell,
                &mut commands,
                &mut color_query,
                &cell_colors,
                &mut board,
                &mut ev_cell_update,
            );
        }
    }
    for ev in ev_mouse_right_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Just)
    {
        if let Ok((_ec, _cell, mut nc)) = cell_query.get_mut(ev.entity) {
            nc.special_hint = !nc.special_hint;
            ev_cell_update.send(CellUpdateEvent);
        }
    }
    for ev in ev_mouse_middle_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Pressed)
    {
        if let Ok((mut ec, mut cell, _nc)) = cell_query.get_mut(ev.entity) {
            commands.entity(ev.entity).remove::<NumberCell>();
            unset_cell(
                &mut commands,
                ev.entity,
                (&mut cell, &mut ec),
                &mut color_query,
                &cell_colors,
                &mut board,
                &mut ev_cell_update,
            );
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
    mut board: ResMut<Board>,
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
                    board.cells[cell.y as usize][cell.x as usize].0 =
                        Some(CellType::NumberCell(HintType::Connected));
                } else {
                    ts.style.color = Color::RED;
                    board.cells[cell.y as usize][cell.x as usize].0 =
                        Some(CellType::NumberCell(HintType::Seperated));
                }
            } else {
                board.cells[cell.y as usize][cell.x as usize].0 =
                    Some(CellType::NumberCell(HintType::None));
            }
            *text_query.get_mut(nc.label).unwrap() =
                Text::from_section(&format!("{}", count), ts.style)
                    .with_alignment(text_settings.alignment);
        }
    }
}

pub fn hotkey_system(
    mut commands: Commands,
    mut cell_query: Query<(&mut Cell, &mut EditorCell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    cell_colors: Res<CellColors>,
    mut board: ResMut<Board>,
    keys: Res<Input<KeyCode>>,
    mut ev_cell_update: EventWriter<CellUpdateEvent>,
) {
    if keys.just_pressed(KeyCode::S) && keys.pressed(KeyCode::LControl) {
        let c = board.trim();
        println!(
            "\n{}\n0",
            board_to_string(BoardConfig {
                width: c[0].len(),
                height: c.len(),
                cells: c,
                hints: Vec::new(),
                text: None,
            })
        );
    }
    if keys.just_pressed(KeyCode::H) {
        for (mut cell, mut ec) in cell_query.iter_mut() {
            ec.toggle_hidden(
                &mut cell,
                &mut commands,
                &mut color_query,
                &cell_colors,
                &mut board,
                &mut ev_cell_update,
            );
        }
    }
}

/// On resizing the window, the board is resized too
/// i.e. the camera zoom (scale) is recalculated
pub fn window_resize_system(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    board: Res<Board>,
    mut ev_window_resize: EventReader<WindowResized>,
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
