use bevy::prelude::{Bundle, ColorMaterial, ColorMesh2dBundle, Component, Handle};

#[derive(Component)]
pub struct MainCamera;

#[derive(Bundle)]
pub struct CellBundle {
    cell: Cell,
    #[bundle]
    color: ColorMesh2dBundle,
}

#[cfg_attr(feature = "debug", derive(bevy_inspector_egui::Inspectable))]
#[derive(Debug, Component, Clone, Copy)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Component)]
pub struct HiddenCell;

#[derive(Debug, Component)]
pub struct NumberCell {
    pub count: u8,
}

#[derive(Debug, Component)]
pub struct EmptyCell;

#[derive(Debug, Component)]
pub struct CellInner;
#[derive(Debug, Component)]
pub struct CellOuter;

pub struct CellColors {
    pub white: Handle<ColorMaterial>,
    pub yellow_dark: Handle<ColorMaterial>,
    pub yellow_medium: Handle<ColorMaterial>,
    pub yellow_light: Handle<ColorMaterial>,
    pub gray_dark: Handle<ColorMaterial>,
    pub gray_light: Handle<ColorMaterial>,
    pub blue_dark: Handle<ColorMaterial>,
    pub blue_light: Handle<ColorMaterial>,
}
