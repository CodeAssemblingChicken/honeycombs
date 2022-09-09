use crate::{
    assets::LocaleAsset,
    components::TextSectionConfig,
    constants::{GameColor, MED_SCALE, RADIUS},
    states::AppState,
};
use bevy::{
    asset::HandleUntyped,
    ecs::world::{Mut, World},
    prelude::{shape::RegularPolygon, AssetServer, Assets, Color, FromWorld, Handle, Mesh},
    sprite::ColorMaterial,
    text::{TextAlignment, TextStyle},
};
use bevy_asset_loader::prelude::AssetCollection;
use bevy_kira_audio::AudioSource;
use ron::{
    de::from_reader,
    ser::{to_writer_pretty, PrettyConfig},
};
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Default)]
pub struct LoadState {
    pub next_state: Option<AppState>,
    pub filename: Option<String>,
    pub ids: Option<(u8, u8)>,
}

pub struct CellMeshes {
    pub std_hexagon_back: Handle<Mesh>,
    pub std_hexagon_outer: Handle<Mesh>,
    pub std_hexagon_inner: Handle<Mesh>,
    pub med_hexagon_back: Handle<Mesh>,
    pub med_hexagon_outer: Handle<Mesh>,
    pub med_hexagon_inner: Handle<Mesh>,
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

/// Resource storing the different colors
pub struct GameColors {
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

impl FromWorld for GameColors {
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

/// Resource for hover sfx
#[derive(AssetCollection)]
pub struct SfxAssets {
    #[asset(path = "sfx/hover.ogg")]
    pub sfx_hover: Handle<AudioSource>,
}

// impl FromWorld for SfxAssets {
//     fn from_world(world: &mut bevy::prelude::World) -> Self {
//         let asset_server = world.get_resource::<AssetServer>().unwrap();
//         let sfx_hover: Handle<AudioSource> = asset_server.load("sfx/hover.ogg");
//         Self { sfx_hover }
//     }
// }

/// Resource for text
#[derive(Clone)]
pub struct TextSettings {
    pub style_cell: TextStyle,
    pub style_cell_large: TextStyle,
    pub style_menu_dark: TextStyle,
    pub alignment: TextAlignment,
}

impl FromWorld for TextSettings {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let font = asset_server.load("fonts/Harabara-dash.ttf");
        let font2 = asset_server.load("fonts/Uroob-Regular.ttf");
        let style_cell = TextStyle {
            font: font.clone(),
            font_size: (RADIUS * 0.75).round(),
            color: Color::WHITE,
        };
        let style_cell_large = TextStyle {
            font,
            font_size: (RADIUS * 0.9).round(),
            color: Color::WHITE,
        };
        let style_menu_dark = TextStyle {
            font: font2,
            font_size: (RADIUS).round(),
            color: Color::BLACK,
        };
        Self {
            style_cell,
            style_cell_large,
            style_menu_dark,
            alignment: TextAlignment::CENTER,
        }
    }
}

#[derive(Default, AssetCollection)]
pub struct LocaleAssets {
    // handles: HashMap<String, Handle<LocaleAsset>>,
    #[asset(path = "lang/en.lang")]
    en: Handle<LocaleAsset>,
    #[asset(path = "lang/de.lang")]
    de: Handle<LocaleAsset>,
    #[asset(path = "lang/fr.lang")]
    fr: Handle<LocaleAsset>,
    #[asset(path = "lang/es.lang")]
    es: Handle<LocaleAsset>,
}
impl LocaleAssets {
    // pub fn load_langs(&mut self, langs: &[&str], asset_server: &AssetServer) {
    //     langs.into_iter().for_each(|l| {
    //         self.handles.insert(
    //             l.to_string(),
    //             asset_server.load(&format!("lang/{}.lang", l)),
    //         );
    //     })
    // }
    pub fn get_string<'a>(
        &'a self,
        key: &str,
        locale_assets: &'a Assets<LocaleAsset>,
        profile: &Profile,
    ) -> Option<&String> {
        if let Some(la) = locale_assets.get(&self.get_handle(profile)) {
            la.strings.get(key)
        } else {
            None
        }
    }
    pub fn get_text_section<'a>(
        &'a self,
        key: &str,
        locale_assets: &'a Assets<LocaleAsset>,
        profile: &Profile,
    ) -> Option<&Vec<TextSectionConfig>> {
        if let Some(la) = locale_assets.get(&self.get_handle(profile)) {
            la.text_sections.get(key)
        } else {
            None
        }
    }
    pub fn get_handle(&self, profile: &Profile) -> Handle<LocaleAsset> {
        match profile.lang.as_str() {
            "de" => self.de.clone_weak(),
            "fr" => self.fr.clone_weak(),
            "es" => self.es.clone_weak(),
            _ => self.en.clone_weak(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Profile {
    pub lang: String,
    pub sfx_volume: f32,
    pub level_points: [[Option<u16>; 6]; 6],
}
impl Profile {
    pub fn new() -> Self {
        if let Ok(file) = File::open("./settings.ron") {
            from_reader(file).unwrap_or_default()
        } else {
            Self::default()
        }
    }
    pub fn get_points(&self) -> u16 {
        self.level_points
            .iter()
            .map(|stage| stage.iter().map(|pts| pts.unwrap_or(0)).sum::<u16>())
            .sum()
    }
    pub fn save(&self) {
        // to_writer_pretty(
        //     File::create("./settings.ron").expect("Failed opening file"),
        //     self,
        //     PrettyConfig::new()
        //         .depth_limit(2)
        //         .separate_tuple_members(true)
        //         .enumerate_arrays(true),
        // )
        // .expect("Error saving profile");
    }
    pub fn update_point(
        &mut self,
        points: u16,
        stage_id: impl Into<usize> + std::marker::Copy,
        level_id: impl Into<usize> + std::marker::Copy,
    ) {
        let current = self.level_points[stage_id.into()][level_id.into()].unwrap_or_default();
        if points > current {
            self.level_points[stage_id.into()][level_id.into()] = Some(points);
            self.save();
        }
    }
}
impl Default for Profile {
    fn default() -> Self {
        Self {
            lang: "en".to_string(),
            sfx_volume: 0.5,
            level_points: Default::default(),
        }
    }
}
