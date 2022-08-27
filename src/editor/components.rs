use bevy::prelude::{Component, Entity};

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CellType {
    NumberCell(HintType),
    EmptyCell,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HintType {
    NONE,
    // SOME is quite ugly, it is used in parsing to indicate that the hint
    // is special and the concrete specialization (CONNECTED or SEPERATED)
    // must first be calculated
    // TODO: Think of something better
    SOME,
    CONNECTED,
    SEPERATED,
}
