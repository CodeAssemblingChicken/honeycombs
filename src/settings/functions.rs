use bevy::{prelude::Assets, text::Text};

use crate::{
    assets::LocaleAsset,
    resources::{LocaleAssets, Profile, TextSettings},
};

pub fn window_mode_text(
    locale: &LocaleAssets,
    locales: &Assets<LocaleAsset>,
    profile: &Profile,
    text_settings: &TextSettings,
) -> Text {
    Text::from_section(
        locale
            .get_string(
                if profile.fullscreen {
                    "fullscreen"
                } else {
                    "windowed"
                },
                &locales,
                &profile,
            )
            .unwrap_or(&"String not found".to_string()),
        text_settings.style_menu_dark.clone(),
    )
    .with_alignment(text_settings.alignment)
}
