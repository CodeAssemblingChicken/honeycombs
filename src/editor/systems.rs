use super::{
    board::Board,
    components::{CellUpdateEvent, EditorCell, EmptyCell, NumberCell, UnsetCell},
    functions::{set_empty_cell, set_number_cell, unset_cell},
};
use crate::{
    board_functions::{count_empty_cells, empty_connected, get_neighbours},
    components::{Cell, RootComponent},
    enums::{CellType, HintType},
    functions::{rescale_board, switch_state},
    parser::board_to_string,
    resources::{GameColors, LoadState, TextSettings},
    states::AppState,
    structs::BoardConfig,
};
use bevy::{
    input::Input,
    prelude::{
        Color, Commands, Entity, EventReader, EventWriter, Handle, KeyCode, Query, Res, ResMut,
        State, Transform, With,
    },
    sprite::ColorMaterial,
    text::Text,
    window::WindowResized,
};
use interactable::components::{
    Entered, Exited, JustPressedLeft, JustPressedRight, PressedLeft, PressedMiddle, PressedRight,
};

type McUnset<'a> = (
    Entity,
    &'a mut EditorCell,
    &'a mut Cell,
    Option<&'a PressedLeft>,
    Option<&'a PressedRight>,
);
pub fn mouse_click_unset_cell(
    mut commands: Commands,
    mut cell_query: Query<McUnset, With<UnsetCell>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    mut board: ResMut<Board>,
    mut ev_cell_update: EventWriter<CellUpdateEvent>,
) {
    for (e, mut ec, mut cell, left, right) in cell_query.iter_mut() {
        if left.is_some() {
            set_empty_cell(
                &mut commands,
                e,
                (&mut cell, &mut ec),
                &mut color_query,
                &game_colors,
                &mut board,
                &mut ev_cell_update,
            )
        } else if right.is_some() {
            set_number_cell(
                &mut commands,
                e,
                (&mut cell, &mut ec),
                &mut color_query,
                &game_colors,
                &mut board,
                &mut ev_cell_update,
            );
        }
    }
}

type McEmpty<'a> = (
    Entity,
    &'a mut EditorCell,
    &'a mut Cell,
    Option<&'a JustPressedLeft>,
    Option<&'a PressedMiddle>,
);
pub fn mouse_click_empty_cell(
    mut commands: Commands,
    mut cell_query: Query<McEmpty, With<EmptyCell>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    mut board: ResMut<Board>,
    mut ev_cell_update: EventWriter<CellUpdateEvent>,
) {
    for (e, mut ec, mut cell, left, middle) in cell_query.iter_mut() {
        if left.is_some() {
            ec.toggle_hidden(
                &mut cell,
                &mut commands,
                &mut color_query,
                &game_colors,
                &mut board,
                &mut ev_cell_update,
            );
        } else if middle.is_some() {
            commands.entity(e).remove::<EmptyCell>();
            unset_cell(
                &mut commands,
                e,
                (&mut cell, &mut ec),
                &mut color_query,
                &game_colors,
                &mut board,
                &mut ev_cell_update,
            );
        }
    }
}

type McNumber<'a> = (
    Entity,
    &'a mut EditorCell,
    &'a mut Cell,
    &'a mut NumberCell,
    Option<&'a JustPressedLeft>,
    Option<&'a JustPressedRight>,
    Option<&'a PressedMiddle>,
);
pub fn mouse_click_number_cell(
    mut commands: Commands,
    mut cell_query: Query<McNumber>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    mut board: ResMut<Board>,
    mut ev_cell_update: EventWriter<CellUpdateEvent>,
) {
    for (e, mut ec, mut cell, mut nc, left, right, middle) in cell_query.iter_mut() {
        if left.is_some() {
            ec.toggle_hidden(
                &mut cell,
                &mut commands,
                &mut color_query,
                &game_colors,
                &mut board,
                &mut ev_cell_update,
            );
        } else if right.is_some() {
            nc.special_hint = !nc.special_hint;
            ev_cell_update.send(CellUpdateEvent);
        } else if middle.is_some() {
            commands.entity(e).remove::<NumberCell>();
            unset_cell(
                &mut commands,
                e,
                (&mut cell, &mut ec),
                &mut color_query,
                &game_colors,
                &mut board,
                &mut ev_cell_update,
            );
        }
    }
}

/// Calls hover on a cell that is entered by the mouse
pub fn mouse_enter_cell(
    mut commands: Commands,
    mut cell_query: Query<(&EditorCell, &mut Cell), With<Entered>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
) {
    for (ec, mut cell) in cell_query.iter_mut() {
        ec.hover(&mut cell, &mut commands, &mut color_query, &game_colors);
    }
}

/// Calls unhover on a cell that is exited by the mouse
pub fn mouse_exit_cell(
    mut commands: Commands,
    mut cell_query: Query<(&EditorCell, &mut Cell), With<Exited>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
) {
    for (ec, mut cell) in cell_query.iter_mut() {
        ec.unhover(&mut cell, &mut commands, &mut color_query, &game_colors);
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
                    ts.style_cell.color = Color::GREEN;
                    board.cells[cell.y as usize][cell.x as usize].0 =
                        Some(CellType::NumberCell(HintType::Connected));
                } else {
                    ts.style_cell.color = Color::rgb(1.0, 0.4, 0.3);
                    board.cells[cell.y as usize][cell.x as usize].0 =
                        Some(CellType::NumberCell(HintType::Seperated));
                }
            } else {
                board.cells[cell.y as usize][cell.x as usize].0 =
                    Some(CellType::NumberCell(HintType::None));
            }
            *text_query.get_mut(nc.label).unwrap() =
                Text::from_section(&format!("{}", count), ts.style_cell)
                    .with_alignment(text_settings.alignment);
        }
    }
}

pub fn hotkey_system(
    mut commands: Commands,
    mut cell_query: Query<(&mut Cell, &mut EditorCell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    mut keys: ResMut<Input<KeyCode>>,
    (mut board, mut app_state, mut load_state): (
        ResMut<Board>,
        ResMut<State<AppState>>,
        ResMut<LoadState>,
    ),
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
                &game_colors,
                &mut board,
                &mut ev_cell_update,
            );
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        keys.clear_just_pressed(KeyCode::Escape);
        switch_state(None, &mut app_state, &mut load_state);
    }
}

/// On resizing the window, the board is resized too
pub fn window_resize_system(
    mut ev_window_resize: EventReader<WindowResized>,
    mut root_query: Query<&mut Transform, With<RootComponent>>,
    board: Res<Board>,
) {
    for ev in ev_window_resize.iter() {
        if let Ok(mut root) = root_query.get_single_mut() {
            rescale_board(board.width, board.height, 3, ev.width, ev.height, &mut root);
        }
    }
}
