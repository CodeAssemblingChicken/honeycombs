use std::{collections::HashMap, fs::File};

use crate::{
    components::TextSectionConfig,
    constants::{GameColor, MED_SCALE, RADIUS},
};
use bevy::{
    audio::AudioSource,
    prelude::{shape::RegularPolygon, AssetServer, Assets, Color, FromWorld, Handle, Mesh},
    sprite::ColorMaterial,
    text::{TextAlignment, TextStyle},
};
use ron::de::from_reader;
use serde::Deserialize;

#[derive(Debug, Default)]
pub struct LevelFile {
    pub filename: Option<String>,
}

pub struct CellMeshes {
    pub std_hexagon_back: Handle<Mesh>,
    pub std_hexagon_outer: Handle<Mesh>,
    pub std_hexagon_inner: Handle<Mesh>,
    pub med_hexagon_back: Handle<Mesh>,
    pub med_hexagon_outer: Handle<Mesh>,
    pub med_hexagon_inner: Handle<Mesh>,
}

/// Resource storing the different colors
pub struct CellColors {
    pub yellow_dark: Handle<ColorMaterial>,
    pub yellow_medium: Handle<ColorMaterial>,
    pub yellow_light: Handle<ColorMaterial>,
    pub gray_dark: Handle<ColorMaterial>,
    pub gray_medium: Handle<ColorMaterial>,
    pub gray_light: Handle<ColorMaterial>,
    pub blue_dark: Handle<ColorMaterial>,
    pub blue_medium: Handle<ColorMaterial>,
    pub blue_light: Handle<ColorMaterial>,
    pub white: Handle<ColorMaterial>,
    pub alpha0: Handle<ColorMaterial>,
    pub alpha1: Handle<ColorMaterial>,
    pub alpha2: Handle<ColorMaterial>,
}

/// Resource for hover sfx
pub struct SfxHover(pub Handle<AudioSource>);

/// Resource for text
#[derive(Clone)]
pub struct TextSettings {
    pub style: TextStyle,
    pub alignment: TextAlignment,
}

#[derive(Deserialize)]
pub struct Locale {
    pub lang: String,
    pub strings: HashMap<String, Vec<TextSectionConfig>>,
}
impl Locale {
    pub fn new(lang: &str) -> Self {
        Self {
            lang: lang.into(),
            strings: from_reader(
                File::open(format!("assets/text/{}.ron", lang)).expect("Failed opening file"),
            )
            .unwrap(),
        }
    }
    pub fn set_lang(&mut self, lang: &str) {
        self.lang = lang.into();
        self.strings = from_reader(
            File::open(format!("assets/text/{}.ron", lang)).expect("Failed opening file"),
        )
        .unwrap();
    }
    pub fn get(&self, key: &str) -> Option<&Vec<TextSectionConfig>> {
        self.strings.get(key)
    }
}

impl FromWorld for CellMeshes {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        Self {
            std_hexagon_back: meshes.add(Mesh::from(RegularPolygon::new(RADIUS, 6))),
            std_hexagon_outer: meshes.add(Mesh::from(RegularPolygon::new(RADIUS * 0.94, 6))),
            std_hexagon_inner: meshes.add(Mesh::from(RegularPolygon::new(RADIUS * 0.8, 6))),
            med_hexagon_back: meshes.add(Mesh::from(RegularPolygon::new(MED_SCALE * RADIUS, 6))),
            med_hexagon_outer: meshes.add(Mesh::from(RegularPolygon::new(
                MED_SCALE * 0.94 * RADIUS,
                6,
            ))),
            med_hexagon_inner: meshes
                .add(Mesh::from(RegularPolygon::new(MED_SCALE * 0.8 * RADIUS, 6))),
        }
    }
}

impl FromWorld for CellColors {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        Self {
            white: materials.add(ColorMaterial::from(Color::WHITE)),
            yellow_dark: materials.add(ColorMaterial::from(GameColor::YELLOW_DARK)),
            yellow_medium: materials.add(ColorMaterial::from(GameColor::YELLOW_MEDIUM)),
            yellow_light: materials.add(ColorMaterial::from(GameColor::YELLOW_LIGHT)),
            gray_dark: materials.add(ColorMaterial::from(GameColor::GRAY_DARK)),
            gray_medium: materials.add(ColorMaterial::from(GameColor::GRAY_MEDIUM)),
            gray_light: materials.add(ColorMaterial::from(GameColor::GRAY_LIGHT)),
            blue_dark: materials.add(ColorMaterial::from(GameColor::BLUE_DARK)),
            blue_medium: materials.add(ColorMaterial::from(GameColor::BLUE_MEDIUM)),
            blue_light: materials.add(ColorMaterial::from(GameColor::BLUE_LIGHT)),
            alpha0: materials.add(ColorMaterial::from(GameColor::ALPHA_0)),
            alpha1: materials.add(ColorMaterial::from(GameColor::ALPHA_1)),
            alpha2: materials.add(ColorMaterial::from(GameColor::ALPHA_2)),
        }
    }
}

impl FromWorld for SfxHover {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let sfx_hover: Handle<AudioSource> = asset_server.load("sfx/hover.ogg");
        Self(sfx_hover)
    }
}

impl FromWorld for TextSettings {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let font = asset_server.load("fonts/Harabara-dash.ttf");
        let text_style = TextStyle {
            font,
            font_size: (RADIUS * 0.75).round(),
            color: Color::WHITE,
        };
        Self {
            style: text_style,
            alignment: TextAlignment::CENTER,
        }
    }
}
