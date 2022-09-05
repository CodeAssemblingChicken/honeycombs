use super::{
    board::Board,
    components::{EmptyCell, GameCell, MistakesText, NumberCell, RemainingText},
};
use crate::{
    components::{Cell, RootComponent},
    functions::{rescale_board, switch_state},
    overlay::resources::{OverlaySettings, OverlayType},
    resources::{GameColors, LoadState, Profile, SfxAssets, TextSettings},
    states::AppState,
};
use bevy::{
    audio::{Audio, PlaybackSettings},
    input::Input,
    prelude::{
        ColorMaterial, Commands, EventReader, Handle, KeyCode, ParamSet, Query, Res, ResMut, State,
        Transform, With, Without,
    },
    text::Text,
    window::WindowResized,
};
use interactable::{
    click::{ClickType, MouseLeftClickEvent, MouseRightClickEvent},
    hover::{MouseEnterEvent, MouseExitEvent, MouseOverEvent},
};

/// Calls uncover on a cell that is clicked by the mouse
pub fn mouse_click_cell(
    mut commands: Commands,
    mut number_cell_query: Query<(&mut GameCell, &mut Cell, &NumberCell), Without<EmptyCell>>,
    mut empty_cell_query: Query<(&mut GameCell, &mut Cell), With<EmptyCell>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    mut board: ResMut<Board>,
    (mut ev_mouse_left_click, mut ev_mouse_right_click): (
        EventReader<MouseLeftClickEvent>,
        EventReader<MouseRightClickEvent>,
    ),
) {
    for ev in ev_mouse_left_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok((lc, cell, _nc)) = number_cell_query.get(ev.entity) {
            lc.uncover_fail(cell, &mut commands, &mut board);
        }
        if let Ok((mut lc, mut cell)) = empty_cell_query.get_mut(ev.entity) {
            lc.uncover(
                &mut cell,
                &mut commands,
                &mut color_query,
                game_colors.as_ref(),
                None,
                &mut board,
            );
        }
    }
    for ev in ev_mouse_right_click
        .iter()
        .filter(|ev| ev.click_type == ClickType::Released)
    {
        if let Ok((mut lc, mut cell, nc)) = number_cell_query.get_mut(ev.entity) {
            lc.uncover(
                &mut cell,
                &mut commands,
                &mut color_query,
                game_colors.as_ref(),
                Some(nc),
                &mut board,
            );
        }
        if let Ok((lc, cell)) = empty_cell_query.get(ev.entity) {
            lc.uncover_fail(cell, &mut commands, &mut board);
        }
    }
}

/// Calls hover on a cell that is entered by the mouse
pub fn mouse_enter_cell(
    mut commands: Commands,
    mut cell_query: Query<(&GameCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    mut ev_mouse_enter: EventReader<MouseEnterEvent>,
    (audio, sfx_assets, profile): (Res<Audio>, Res<SfxAssets>, Res<Profile>),
) {
    for ev in ev_mouse_enter.iter() {
        if let Ok((lc, mut cell)) = cell_query.get_mut(ev.0) {
            audio.play_with_settings(
                sfx_assets.sfx_hover.clone(),
                PlaybackSettings::ONCE.with_volume(profile.sfx_volume),
            );
            lc.hover(&mut cell, &mut commands, &mut color_query, &game_colors);
        }
    }
}

/// Calls unhover on a cell that is exited by the mouse
pub fn mouse_exit_cell(
    mut commands: Commands,
    mut cell_query: Query<(&GameCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    mut ev_mouse_exit: EventReader<MouseExitEvent>,
) {
    for ev in ev_mouse_exit.iter() {
        if let Ok((lc, mut cell)) = cell_query.get_mut(ev.0) {
            lc.unhover(&mut cell, &mut commands, &mut color_query, &game_colors);
        }
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
            rescale_board(board.width, board.height, 4, ev.width, ev.height, &mut root);
        }
    }
}

pub fn hotkey_system(
    mut app_state: ResMut<State<AppState>>,
    mut keys: ResMut<Input<KeyCode>>,
    mut overlay_settings: ResMut<OverlaySettings>,
    board: Res<Board>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        keys.clear_just_pressed(KeyCode::Escape);
        overlay_settings.stage_id = board.get_stage_id();
        overlay_settings.level_id = board.get_level_id();
        overlay_settings.max_points = board.get_max_points();
        overlay_settings.overlay_type = OverlayType::Pause;
        app_state.push(AppState::Overlay).unwrap();
    }
}

pub fn check_solved(
    mut text_set: ParamSet<(
        Query<&mut Text, With<RemainingText>>,
        Query<&mut Text, With<MistakesText>>,
    )>,
    board: Res<Board>,
    text_settings: Res<TextSettings>,
    mut app_state: ResMut<State<AppState>>,
    mut overlay_settings: ResMut<OverlaySettings>,
    mut profile: ResMut<Profile>,
) {
    if board.is_changed() {
        if let Ok(mut text) = text_set.p0().get_single_mut() {
            *text = Text::from_section(
                format!("{}: {}", "Remaining", board.get_empty_remaining()),
                text_settings.style_cell.clone(),
            );
        }
        if let Ok(mut text) = text_set.p1().get_single_mut() {
            *text = Text::from_section(
                format!("{}: {}", "Mistakes", board.get_mistakes()),
                text_settings.style_cell.clone(),
            );
        }
        if board.is_solved() {
            println!("{},{}", board.get_stage_id(), board.get_level_id());
            profile.update_point(
                board.get_points(),
                board.get_stage_id(),
                board.get_level_id(),
            );
            // switch_state(
            //     Some(AppState::LevelSelection),
            //     &mut app_state,
            //     &mut load_state,
            // );
            overlay_settings.stage_id = board.get_stage_id();
            overlay_settings.level_id = board.get_level_id();
            overlay_settings.max_points = board.get_max_points();
            overlay_settings.points = board.get_points();
            overlay_settings.mistakes = board.get_mistakes();
            overlay_settings.overlay_type = OverlayType::LevelComplete;
            app_state.push(AppState::Overlay).unwrap();
        }
    }
}

pub fn pause(
    mut commands: Commands,
    mut cell_query: Query<(&GameCell, &mut Cell)>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
) {
    for (gc, mut c) in cell_query.iter_mut() {
        gc.unhover(&mut c, &mut commands, &mut color_query, &game_colors);
    }
}
