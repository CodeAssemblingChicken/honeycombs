use bevy::prelude::{Component, Entity};

use crate::components::CellType;

#[derive(Component)]
pub struct EditorCell {
    pub cell_type: CellType,
}

/// Component for the NumberCell type
#[derive(Debug, Component)]
pub struct NumberCell {
    pub count: u8,
    pub label: Entity,
}

/// Component for the EmptyCell type
#[derive(Debug, Component)]
pub struct EmptyCell;

#[derive(Debug, Component)]
pub struct UnsetCell;
