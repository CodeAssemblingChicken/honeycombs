use bevy::prelude::Component;

#[derive(Component)]
pub struct MainCamera;

#[derive(Debug, Component, Clone, Copy)]
pub struct Cell;

#[derive(Debug, Component)]
pub struct HiddenCell;

#[derive(Debug, Component)]
pub struct NumberCell {
    count: u8,
}

#[derive(Debug, Component)]
pub struct EmptyCell;
