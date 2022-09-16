use bevy::{
    math::Vec2,
    prelude::{
        default, shape::Quad, Assets, Bundle, ColorMaterial, ColorMesh2dBundle, Handle, Mesh,
        Transform,
    },
};
use interactable::{components::Interactable, shapes::Shape};

use crate::components::MenuButton;

#[derive(Bundle)]
pub struct MenuButtonBundle {
    interactable: Interactable,
    menu_button: MenuButton,
    #[bundle]
    mesh_bundle: ColorMesh2dBundle,
}

impl MenuButtonBundle {
    pub fn new(
        transform: Transform,
        (width, height): (f32, f32),
        material: Handle<ColorMaterial>,
        meshes: &mut Assets<Mesh>,
    ) -> Self {
        Self {
            mesh_bundle: ColorMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(Quad::new(Vec2::new(width, height))))
                    .into(),
                material,
                transform,
                ..default()
            },
            interactable: Interactable {
                shape: Shape::Quad(interactable::shapes::Quad { width, height }),
                ..default()
            },
            menu_button: MenuButton,
        }
    }
}
