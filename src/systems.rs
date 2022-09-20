use bevy::prelude::{ColorMaterial, Component, Handle, Query, Res, With, Without};
use interactable::components::{Entered, Exited};

use crate::{components::MenuButton, resources::GameColors};

pub fn menu_button_hovered<T: Component>(
    mut entered_query: Query<
        &mut Handle<ColorMaterial>,
        (With<MenuButton>, With<Entered>, With<T>),
    >,
    mut exited_query: Query<
        &mut Handle<ColorMaterial>,
        (With<MenuButton>, With<Exited>, Without<Entered>),
    >,
    game_colors: Res<GameColors>,
) {
    for mut h in entered_query.iter_mut() {
        *h = game_colors.menu_button_hovered.clone();
    }
    for mut h in exited_query.iter_mut() {
        *h = game_colors.menu_button.clone();
    }
}
