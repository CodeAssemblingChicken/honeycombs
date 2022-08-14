use bevy::prelude::{Component, Entity};
use bevy_inspector_egui::Inspectable;

#[derive(Component)]
pub struct MainCamera;

#[derive(Debug, Component, Clone, Copy, Inspectable)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Component)]
pub struct HiddenCell;

#[derive(Debug, Component)]
pub struct NumberCell {
    count: u8,
}

#[derive(Debug, Component)]
pub struct EmptyCell;

#[derive(Debug)]
pub struct HoveredCell {
    pub coords: Option<(u32, u32)>,
    pub entity: Option<Entity>,
}
