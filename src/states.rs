#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Level,
    EndScreen,
    Editor,
    Settings,
    Loading,
}
