use bevy::{
    audio::AudioSource,
    prelude::{shape::RegularPolygon, AssetServer, Assets, Color, FromWorld, Handle, Mesh},
    sprite::ColorMaterial,
    text::{TextAlignment, TextStyle},
};

use crate::constants::RADIUS;

pub struct CellMeshes {
    pub big_hexagon: Handle<Mesh>,
    pub medium_hexagon: Handle<Mesh>,
    pub small_hexagon: Handle<Mesh>,
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

impl FromWorld for CellMeshes {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
        Self {
            big_hexagon: meshes.add(Mesh::from(RegularPolygon::new(RADIUS, 6))),
            medium_hexagon: meshes.add(Mesh::from(RegularPolygon::new(RADIUS * 0.94, 6))),
            small_hexagon: meshes.add(Mesh::from(RegularPolygon::new(RADIUS * 0.8, 6))),
        }
    }
}

impl FromWorld for CellColors {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        Self {
            white: materials.add(ColorMaterial::from(Color::WHITE)),
            yellow_dark: materials.add(ColorMaterial::from(Color::hex("d87408").unwrap())),
            yellow_medium: materials.add(ColorMaterial::from(Color::hex("dc8c10").unwrap())),
            yellow_light: materials.add(ColorMaterial::from(Color::hex("e4a020").unwrap())),
            gray_dark: materials.add(ColorMaterial::from(Color::hex("24221c").unwrap())),
            gray_medium: materials.add(ColorMaterial::from(Color::hex("37352a").unwrap())),
            gray_light: materials.add(ColorMaterial::from(Color::hex("484537").unwrap())),
            blue_dark: materials.add(ColorMaterial::from(Color::hex("0070e4").unwrap())),
            blue_medium: materials.add(ColorMaterial::from(Color::hex("0088e8").unwrap())),
            blue_light: materials.add(ColorMaterial::from(Color::hex("00a0f0").unwrap())),
            alpha0: materials.add(ColorMaterial::from(Color::rgba(1., 1., 1., 0.))),
            alpha1: materials.add(ColorMaterial::from(Color::rgba(1., 1., 1., 0.05))),
            alpha2: materials.add(ColorMaterial::from(Color::rgba(1., 1., 1., 0.15))),
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
