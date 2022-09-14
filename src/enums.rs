/// The type of cell.
/// Used in cell component for uncover-handling
#[cfg_attr(
    feature = "bevy-inspector-egui",
    derive(bevy_inspector_egui::Inspectable)
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    NumberCell(HintType),
    EmptyCell,
}

/// Direction of the column/row hints.
/// Straight down (TOP), down-right (RIGHT) and down-left (LEFT)
#[derive(Debug, Clone, Copy)]
pub enum HintDirection {
    Down,
    LeftDown,
    RightDown,
    Up,
    LeftUp,
    RightUp,
}

/// Indicator for special hints (connected or seperated cells)
#[cfg_attr(
    feature = "bevy-inspector-egui",
    derive(bevy_inspector_egui::Inspectable)
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HintType {
    None,
    // SOME is quite ugly, it is used in parsing to indicate that the hint
    // is special and the concrete specialization (CONNECTED or SEPERATED)
    // must first be calculated
    // TODO: Think of something better
    Some,
    Connected,
    Seperated,
}

/// Required because of bevy_inspector_egui::Inspectable
impl Default for HintType {
    fn default() -> Self {
        Self::None
    }
}
