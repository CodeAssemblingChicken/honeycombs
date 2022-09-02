use bevy::{
    math::Vec3,
    prelude::{Color, Transform},
};

/// Hex radius (i.e. circumcircle)
pub const RADIUS: f32 = 100.0;
pub const MED_SCALE: f32 = 1.5;

/// z coordinates for different items
pub const Z_INDEX_CELL_BACK: f32 = 2.0;
pub const Z_INDEX_CELL_OUTER: f32 = 3.0;
pub const Z_INDEX_CELL_INNER: f32 = 4.0;
pub const Z_INDEX_TEXT: f32 = 10.0;

/// Cell scale values (normal and hovered)
pub const SCALE_NORMAL: Vec3 = Vec3::new(1., 1., 1.);
pub const SCALE_ENLARGED: Vec3 = Vec3::new(1.04, 1.04, 1.);

pub const OUTER_TRANSFORM: Transform =
    Transform::from_translation(Vec3::new(0.0, 0.0, Z_INDEX_CELL_OUTER));
pub const INNER_TRANSFORM: Transform =
    Transform::from_translation(Vec3::new(0.0, 0.0, Z_INDEX_CELL_INNER));

pub struct GameColor;
impl GameColor {
    pub const YELLOW_DARK: Color = Color::rgb(0.847, 0.455, 0.031); // #d87408
    pub const YELLOW_MEDIUM: Color = Color::rgb(0.863, 0.549, 0.063); // #dc8c10
    pub const YELLOW_LIGHT: Color = Color::rgb(0.894, 0.627, 0.125); // #e4a020
    pub const GRAY_DARK: Color = Color::rgb(0.141, 0.133, 0.11); // #24221c
    pub const GRAY_MEDIUM: Color = Color::rgb(0.216, 0.208, 0.165); // #37352a
    pub const GRAY_LIGHT: Color = Color::rgb(0.282, 0.271, 0.216); // #484537
    pub const BLUE_DARK: Color = Color::rgb(0.0, 0.439, 0.894); // #0070e4
    pub const BLUE_MEDIUM: Color = Color::rgb(0.0, 0.533, 0.91); // #0088e8
    pub const BLUE_LIGHT: Color = Color::rgb(0.0, 0.627, 0.941); // #00a0f0
    pub const ALPHA_0: Color = Color::rgba(0.5, 0.5, 0.5, 0.);
    pub const ALPHA_1: Color = Color::rgba(0.5, 0.5, 0.5, 0.1);
    pub const ALPHA_2: Color = Color::rgba(0.5, 0.5, 0.5, 0.2);
}
