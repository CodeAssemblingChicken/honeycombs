#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Home,
    LevelSelection,
    Level,
    Overlay,
    Editor,
    Settings,
    StateChange,
    AssetLoading,
}
