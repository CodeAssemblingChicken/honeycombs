use bevy::{
    ecs::bundle,
    math::Vec2,
    prelude::{
        default, shape::Quad, Assets, Bundle, Color, ColorMaterial, ColorMesh2dBundle, Mesh,
        Transform,
    },
};
use interactable::{components::Interactable, shapes::Shape};

#[derive(Bundle)]
pub struct MenuButtonBundle {
    interactable: Interactable,
    #[bundle]
    mesh_bundle: ColorMesh2dBundle,
}

impl MenuButtonBundle {
    pub fn new(
        transform: Transform,
        (width, height): (f32, f32),
        color: Color,
        meshes: &mut Assets<Mesh>,
        colors: &mut Assets<ColorMaterial>,
    ) -> Self {
        Self {
            mesh_bundle: ColorMesh2dBundle {
                mesh: meshes
                    .add(Mesh::from(Quad::new(Vec2::new(width, height))))
                    .into(),
                material: colors.add(ColorMaterial::from(color)),
                transform,
                ..default()
            },
            interactable: Interactable {
                shape: Shape::Quad(interactable::shapes::Quad { width, height }),
                ..default()
            },
        }
    }
}
