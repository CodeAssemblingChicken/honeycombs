use super::{
    board::Board,
    components::{EmptyCell, GameCell, MistakesText, NumberCell, RemainingText},
};
use crate::{
    components::{Cell, ColumnHint, RootComponent},
    functions::rescale_board,
    overlay::resources::{OverlaySettings, OverlayType},
    resources::{GameColors, Profile, SfxAssets, TextSettings},
    states::AppState,
};
use bevy::{
    hierarchy::Children,
    input::Input,
    prelude::{
        ColorMaterial, Commands, EventReader, Handle, KeyCode, ParamSet, Query, Res, ResMut, State,
        Transform, Visibility, With, Without,
    },
    text::Text,
    window::WindowResized,
};
use bevy_kira_audio::{Audio, AudioControl};
use interactable::components::{Entered, Exited, ReleasedLeft, ReleasedRight};

type McNumberCell<'a> = (
    &'a mut GameCell,
    &'a mut Cell,
    &'a NumberCell,
    Option<&'a ReleasedLeft>,
    Option<&'a ReleasedRight>,
);
type McEmptyCell<'a> = (
    &'a mut GameCell,
    &'a mut Cell,
    Option<&'a ReleasedLeft>,
    Option<&'a ReleasedRight>,
);
/// Calls uncover on a cell that is clicked by the mouse
pub fn mouse_click_cell(
    mut commands: Commands,
    mut number_cell_query: Query<McNumberCell, Without<EmptyCell>>,
    mut empty_cell_query: Query<McEmptyCell, With<EmptyCell>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    (game_colors, profile): (Res<GameColors>, Res<Profile>),
    mut board: ResMut<Board>,
) {
    for (mut lc, mut cell, nc, left, right) in number_cell_query.iter_mut() {
        let fail =
            left.is_some() && !profile.mouse_inverted || right.is_some() && profile.mouse_inverted;
        let ok =
            left.is_some() && profile.mouse_inverted || right.is_some() && !profile.mouse_inverted;
        if fail {
            lc.uncover_fail(&cell, &mut commands, &mut board);
        } else if ok {
            lc.uncover(
                &mut cell,
                &mut commands,
                &mut color_query,
                game_colors.as_ref(),
                Some(nc),
                &mut board,
            );
        }
    }
    for (mut lc, mut cell, left, right) in empty_cell_query.iter_mut() {
        let fail =
            right.is_some() && !profile.mouse_inverted || left.is_some() && profile.mouse_inverted;
        let ok =
            right.is_some() && profile.mouse_inverted || left.is_some() && !profile.mouse_inverted;
        if fail {
            lc.uncover_fail(&cell, &mut commands, &mut board);
        } else if ok {
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
}

/// Calls hover on a cell that is entered by the mouse
pub fn mouse_enter_cell(
    mut commands: Commands,
    mut cell_query: Query<(&GameCell, &mut Cell), With<Entered>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
    (audio, sfx_assets, profile): (Res<Audio>, Res<SfxAssets>, Res<Profile>),
) {
    for (lc, mut cell) in cell_query.iter_mut() {
        audio
            .play(sfx_assets.sfx_hover.clone())
            .with_volume(profile.sfx_volume as f64);
        lc.hover(&mut cell, &mut commands, &mut color_query, &game_colors);
    }
}

/// Calls unhover on a cell that is exited by the mouse
pub fn mouse_exit_cell(
    mut commands: Commands,
    mut cell_query: Query<(&GameCell, &mut Cell), With<Exited>>,
    mut color_query: Query<&mut Handle<ColorMaterial>>,
    game_colors: Res<GameColors>,
) {
    for (lc, mut cell) in cell_query.iter_mut() {
        lc.unhover(&mut cell, &mut commands, &mut color_query, &game_colors);
    }
}

pub fn mouse_click_hint(
    hint_query: Query<&Children, (With<ColumnHint>, With<ReleasedLeft>)>,
    mut hint_line_query: Query<&mut Visibility>,
) {
    for hint in hint_query.iter() {
        for line in hint.iter() {
            if let Ok(mut visibility) = hint_line_query.get_mut(*line) {
                visibility.is_visible = !visibility.is_visible;
            }
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
