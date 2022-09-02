#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Level,
    Overlay,
    Editor,
    Settings,
    Loading,
}
