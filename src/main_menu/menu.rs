use crate::{
    components::{Cell, CellInner, CellOuter},
    constants::{RADIUS, Z_INDEX_CELL_BACK, Z_INDEX_CELL_INNER, Z_INDEX_CELL_OUTER},
    main_menu::components::LevelSelectionCell,
    resources::{CellColors, CellMeshes},
};
use bevy::{
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    math::Vec3,
    prelude::{default, Camera, Commands, Entity, Query, Res, Transform, Without},
    sprite::ColorMesh2dBundle,
};
use interactable::{
    click::Clickable,
    hover::Hoverable,
    shapes::{Hexagon, Shape},
};

pub fn setup(mut commands: Commands, cell_meshes: Res<CellMeshes>, cell_colors: Res<CellColors>) {
    let radius = RADIUS;

    let medium_transform = Transform::from_translation(Vec3::new(0.0, 0.0, Z_INDEX_CELL_OUTER));
    let small_transform = Transform::from_translation(Vec3::new(0.0, 0.0, Z_INDEX_CELL_INNER));

    for x in 1..=2 {
        let mut big_transform = Transform::from_translation(Vec3::new(
            (x as f32 - 1.5) * 5. * RADIUS,
            0.,
            Z_INDEX_CELL_BACK,
        ));
        big_transform.rotate_z(f32::to_radians(90.0));

        let b1 = ColorMesh2dBundle {
            mesh: cell_meshes.medium_hexagon.clone().into(),
            material: cell_colors.blue_medium.clone(),
            transform: medium_transform,
            ..default()
        };
        let b2 = ColorMesh2dBundle {
            mesh: cell_meshes.small_hexagon.clone().into(),
            material: cell_colors.blue_light.clone(),
            transform: small_transform,
            ..default()
        };

        // do the same for the child
        let child1 = commands.spawn_bundle(b1).insert(CellOuter).id();
        let child2 = commands.spawn_bundle(b2).insert(CellInner).id();

        let cell = commands
            .spawn()
            .insert_bundle(ColorMesh2dBundle {
                mesh: cell_meshes.big_hexagon.clone().into(),
                material: cell_colors.white.clone(),
                transform: big_transform,
                ..default()
            })
            .id();
        commands.entity(cell).push_children(&[child1, child2]);

        commands
            .entity(cell)
            .insert(Hoverable {
                ignore_scale: true,
                shape: Shape::Hexagon(Hexagon {
                    radius: radius,
                    point_up: false,
                }),
                ..default()
            })
            .insert(Clickable {
                ignore_scale: true,
                shape: Shape::Hexagon(Hexagon {
                    radius: radius,
                    point_up: false,
                }),
                left_released: true,
                ..default()
            });

        let cell_component = Cell {
            x: 0,
            y: 0,
            entity: cell,
            outer_hexagon: child1,
            inner_hexagon: child2,
            orig: big_transform,
            hovering: false,
        };
        commands
            .entity(cell)
            .insert(cell_component)
            .insert(LevelSelectionCell { stage: 1, level: x });
    }
}

pub fn cleanup(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}
