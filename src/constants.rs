use bevy::math::Vec3;

/// Hex radius (i.e. circumcircle)
pub const RADIUS: f32 = 100.0;

/// z coordinates for different items
pub const Z_INDEX_CELL_BACK: f32 = 2.0;
pub const Z_INDEX_CELL_OUTER: f32 = 3.0;
pub const Z_INDEX_CELL_INNER: f32 = 4.0;
pub const Z_INDEX_TEXT: f32 = 10.0;

/// Cell scale values (normal and hovered)
pub const SCALE_NORMAL: Vec3 = Vec3::new(1., 1., 1.);
pub const SCALE_ENLARGED: Vec3 = Vec3::new(1.04, 1.04, 1.);
