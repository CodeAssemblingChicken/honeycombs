#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    MainMenu,
    Level,
    EndScreen,
    Editor,
    Settings,
    Loading,
}
