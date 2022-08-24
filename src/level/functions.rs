use bevy::{
    prelude::{default, Commands, Entity, Transform},
    text::{Text, Text2dBundle},
};

use crate::Z_INDEX_TEXT;

use super::resources::TextSettings;

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
