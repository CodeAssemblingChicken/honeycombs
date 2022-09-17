use bevy::prelude::Component;

#[derive(Component)]
pub struct UiBackground;

#[derive(Component)]
pub struct UiRootNode;

#[derive(Component)]
pub struct ContentPane;

#[derive(Component)]
pub struct ButtonMenu;
#[derive(Component)]
pub struct ButtonRestart;
#[derive(Component)]
pub struct ButtonVariable(pub bool);
