#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppState {
    AssetLoading,
    Editor,
    #[default]
    Home,
    Level,
    LevelSelection,
    Options,
    Overlay,
    StateChange,
    Quit,
}
