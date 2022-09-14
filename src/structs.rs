use crate::{components::ColumnHint, enums::CellType};
use bevy::{
    prelude::Color,
    text::{TextSection, TextStyle},
};
use serde::Deserialize;

/// Used to pass configuration from parser to board
pub struct BoardConfig {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<(Option<CellType>, bool)>>,
    pub hints: Vec<ColumnHint>,
    pub text: Option<(i32, i32, String)>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TextSectionConfig {
    pub text: String,
    pub color: Option<Color>,
    pub interactable: bool,
}

impl TextSectionConfig {
    pub fn new(text: impl Into<String>, color: Option<Color>, interactable: bool) -> Self {
        Self {
            text: text.into(),
            color,
            interactable,
        }
    }
    pub fn to_text_section(&self, text_style: &TextStyle) -> TextSection {
        let mut ts = text_style.clone();
        if let Some(color) = self.color {
            ts.color = color;
        }
        TextSection::new(self.text.clone(), ts)
    }
}
