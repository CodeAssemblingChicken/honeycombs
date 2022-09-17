#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppState {
    AssetLoading,
    Credits,
    Dialog,
    Editor,
    #[default]
    Home,
    Level,
    LevelSelection,
    Options,
    Overlay,
    StateChange,
    Tutorial,
    Quit,
}
