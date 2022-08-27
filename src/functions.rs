use crate::{
    constants::{RADIUS, Z_INDEX_TEXT},
    resources::TextSettings,
};
use bevy::{
    math::Vec3,
    prelude::{default, Camera, Commands, Entity, Query, Transform, With},
    text::{Text, Text2dBundle},
};

/// Spawns the text in a cell
pub fn spawn_cell_text(
    commands: &mut Commands,
    text: &str,
    text_settings: &TextSettings,
) -> Entity {
    let mut t = Transform::identity();
    t.translation.z = Z_INDEX_TEXT;
    t.rotate_z(f32::to_radians(-90.0));
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::from_section(text, text_settings.style.clone())
                .with_alignment(text_settings.alignment),
            transform: t,
            ..default()
        })
        .id()
}

pub fn rescale_board(
    board_width: usize,
    board_height: usize,
    margin: usize,
    wd_width: f32,
    wd_height: f32,
    camera_query: &mut Query<&mut Transform, With<Camera>>,
) {
    let w = ((board_width + margin) as f32 * RADIUS * 1.56) / wd_width;
    let h = ((board_height + margin) as f32 * RADIUS * 1.8) / wd_height;
    let s = w.max(h);
    for mut t in camera_query.iter_mut() {
        t.scale = Vec3::new(s, s, 1.0);
    }
}
