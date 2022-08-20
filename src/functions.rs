use bevy::{
    prelude::{default, Commands, Transform},
    text::{Text, Text2dBundle},
};

use crate::{
    components::{NumberCell, TextSettings},
    Z_INDEX_TEXT,
};

pub fn spawn_cell_text(
    orig: Transform,
    commands: &mut Commands,
    number_cell: &NumberCell,
    text_settings: &TextSettings,
) {
    let mut t = orig.clone();
    t.translation.z = Z_INDEX_TEXT;
    t.rotation.z = 0.0;
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section(
            format!("{}", number_cell.count),
            text_settings.style.clone(),
        )
        .with_alignment(text_settings.alignment),
        transform: t,
        ..default()
    });
}
