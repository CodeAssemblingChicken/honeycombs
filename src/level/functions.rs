use super::board::Board;
use crate::{
    constants::{RADIUS, Z_INDEX_TEXT},
    resources::TextSettings,
};
use bevy::{
    math::Vec3,
    prelude::{default, Camera, Commands, Entity, Query, Transform, With},
    text::{Text, Text2dBundle},
};

/// Spawns the text in a number cell
pub fn spawn_cell_text(
    orig: Transform,
    commands: &mut Commands,
    count: u8,
    text_settings: &TextSettings,
) -> Entity {
    let mut t = orig.clone();
    t.translation.z = Z_INDEX_TEXT;
    t.rotation.z = 0.0;
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(format!("{}", count), text_settings.style.clone())
                .with_alignment(text_settings.alignment),
            transform: t,
            ..default()
        })
        .id()
}

pub fn rescale_board(
    board: &Board,
    wd_width: f32,
    wd_height: f32,
    camera_query: &mut Query<&mut Transform, With<Camera>>,
) {
    let w = ((board.width + 4) as f32 * RADIUS * 1.56) / wd_width;
    let h = ((board.height + 4) as f32 * RADIUS * 1.8) / wd_height;
    let s = w.max(h);
    for mut t in camera_query.iter_mut() {
        t.scale = Vec3::new(s, s, 1.0);
    }
}
