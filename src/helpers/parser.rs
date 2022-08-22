use std::fs;

use crate::components::CellType;

pub fn board_from_file(file_name: &str) -> Vec<Vec<(Option<CellType>, bool)>> {
    let mut result = Vec::new();
    let mut len = usize::MAX;

    for line in fs::read_to_string(file_name)
        .expect("Should have been able to read the file")
        .lines()
        .rev()
    {
        if len == usize::MAX {
            len = line.len();
            assert!(len > 0, "Width must at least be one.")
        } else {
            assert!(len == line.len(), "Lines must have same length.")
        }
        result.push(parse_line(line))
    }
    return result;
}

fn parse_line(line: &str) -> Vec<(Option<CellType>, bool)> {
    let mut cells = Vec::new();
    for c in line.chars() {
        match c {
            '0' => cells.push((Some(CellType::NumberCell), true)),
            '1' => cells.push((Some(CellType::EmptyCell), true)),
            '2' => cells.push((Some(CellType::NumberCell), false)),
            '3' => cells.push((Some(CellType::EmptyCell), false)),
            _ => cells.push((None, false)),
        }
    }
    return cells;
}
