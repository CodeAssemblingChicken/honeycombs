use bevy::{
    audio::AudioSource,
    prelude::Handle,
    sprite::ColorMaterial,
    text::{TextAlignment, TextStyle},
};

/// Resource storing the different colors
pub struct CellColors {
    pub white: Handle<ColorMaterial>,
    pub yellow_dark: Handle<ColorMaterial>,
    // TODO: This name is super inconsistent
    pub yellow_medium: Handle<ColorMaterial>,
    pub yellow_light: Handle<ColorMaterial>,
    pub gray_dark: Handle<ColorMaterial>,
    pub gray_light: Handle<ColorMaterial>,
    pub blue_dark: Handle<ColorMaterial>,
    pub blue_light: Handle<ColorMaterial>,
}

/// Resource for hover sfx
pub struct SfxHover(pub Handle<AudioSource>);

/// Resource for text
#[derive(Clone)]
pub struct TextSettings {
    pub style: TextStyle,
    pub alignment: TextAlignment,
}
