use bevy::{
    math::Vec3,
    prelude::{shape::RegularPolygon, Assets, Commands, Mesh, ResMut, Transform},
};

use crate::constants::{RADIUS, Z_INDEX_CELL_OUTER};

fn setup_menu(commands: &mut Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let medium_transform = Transform::from_translation(Vec3::new(0.0, 0.0, Z_INDEX_CELL_OUTER));
    let small_transform = Transform::from_translation(Vec3::new(0.0, 0.0, Z_INDEX_CELL_OUTER));

    let big_hexagon = meshes.add(Mesh::from(RegularPolygon::new(RADIUS, 6)));
    let medium_hexagon = meshes.add(Mesh::from(RegularPolygon::new(RADIUS * 0.94, 6)));
    let small_hexagon = meshes.add(Mesh::from(RegularPolygon::new(RADIUS * 0.8, 6)));
}
