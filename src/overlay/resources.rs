/// May be extended in the future
#[derive(Default)]
pub struct OverlaySettings {
    pub stage_id: u8,
    pub level_id: u8,
    pub max_points: u16,
    pub points: u16,
    pub mistakes: u16,
    pub overlay_type: OverlayType,
}

#[derive(Default, PartialEq, Eq)]
pub enum OverlayType {
    Pause,
    #[default]
    LevelComplete,
}
