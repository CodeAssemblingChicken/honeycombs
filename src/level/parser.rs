use super::board::BoardConfig;
use crate::components::{CellType, ColumnHint, HintDirection, HintType};
use std::fs;

const DONT_MESS: &str = "Please don't mess with my files";
const EXPECTED_NO: &str = "Expected a number";
const TOO_FEW_ARGS: &str = "Expected more arguments";

/// Receives a file and creates a BoardConfig from it
pub fn board_from_file(filename: &str) -> BoardConfig {
    let mut cells = Vec::new();
    let file =
        fs::read_to_string(filename).unwrap_or_else(|_| panic!("File \"{}\" not found!", filename));
    let mut lines = file.lines();
    let mut line_no = 0;
    let (w, h) = parse_tuple(
        lines
            .next()
            .unwrap_or_else(|| panic!("{} in line {}", DONT_MESS, line_no)),
        line_no,
    );
    line_no += 1;
    assert!(h > 0, "Height must at least be 1.");
    assert!(w > 0, "Width must at least be 1.");

    (0..h).into_iter().for_each(|_| {
        let l = lines
            .next()
            .unwrap_or_else(|| panic!("{} in line {}", DONT_MESS, line_no));
        assert!(l.len() == w, "Lines must have specified width: {}", w,);
        cells.push(parse_grid_row(l));
        line_no += 1;
    });

    let mut hints = Vec::new();

    let num_hints: usize = lines
        .next()
        .unwrap_or_else(|| panic!("{} in line {}", DONT_MESS, line_no))
        .parse()
        .unwrap_or_else(|_| panic!("{} in line {}", EXPECTED_NO, line_no));
    line_no += 1;
    (0..num_hints).into_iter().for_each(|_| {
        let l = lines
            .next()
            .unwrap_or_else(|| panic!("{} in line {}", DONT_MESS, line_no));
        hints.push(parse_hint(l, line_no));
        line_no += 1;
    });
    BoardConfig { cells, hints }
}

/// Function to parse a numeric tuple in a file
fn parse_tuple(line: &str, line_no: usize) -> (usize, usize) {
    let mut split = line.split(',');
    let s = split
        .next()
        .unwrap_or_else(|| panic!("{} in line {}", TOO_FEW_ARGS, line_no));
    let x = s
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("{} in line {}", EXPECTED_NO, line_no));
    let s = split
        .next()
        .unwrap_or_else(|| panic!("{} in line {}", TOO_FEW_ARGS, line_no));
    let y = s
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("{} in line {}", EXPECTED_NO, line_no));
    (x, y)
}

/// Function to parse a column-hint in a file
fn parse_hint(line: &str, line_no: usize) -> ColumnHint {
    let mut split = line.split(',');
    let s = split
        .next()
        .unwrap_or_else(|| panic!("{} in line {}", TOO_FEW_ARGS, line_no));
    let x = s
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("{} in line {}", EXPECTED_NO, line_no));
    let s = split
        .next()
        .unwrap_or_else(|| panic!("{} in line {}", TOO_FEW_ARGS, line_no));
    let y = s
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("{} in line {}", EXPECTED_NO, line_no));
    let s = split
        .next()
        .unwrap_or_else(|| panic!("{} in line {}", TOO_FEW_ARGS, line_no));
    let hint_dir: i8 = s
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("{} in line {}", EXPECTED_NO, line_no));
    let s = split
        .next()
        .unwrap_or_else(|| panic!("{} in line {}", TOO_FEW_ARGS, line_no));
    let hint_type = s
        .trim()
        .parse()
        .unwrap_or_else(|_| panic!("{} in line {}", EXPECTED_NO, line_no));
    ColumnHint {
        x,
        y,
        dir: match hint_dir {
            -1 => HintDirection::LeftDown,
            1 => HintDirection::RightDown,
            -2 => HintDirection::LeftUp,
            2 => HintDirection::RightUp,
            3 | -3 => HintDirection::Up,
            _ => HintDirection::Down,
        },
        hint_type: match hint_type {
            0 => HintType::None,
            _ => HintType::Some,
        },
    }
}

/// Function to parse a line of a file to a row in the grid
fn parse_grid_row(line: &str) -> Vec<(Option<CellType>, bool)> {
    let mut cells = Vec::new();
    for c in line.chars() {
        match c {
            '0' => cells.push((Some(CellType::EmptyCell), true)),
            '1' => cells.push((Some(CellType::EmptyCell), false)),
            '2' => cells.push((Some(CellType::NumberCell(HintType::None)), true)),
            '3' => cells.push((Some(CellType::NumberCell(HintType::None)), false)),
            '4' => cells.push((Some(CellType::NumberCell(HintType::Some)), true)),
            '5' => cells.push((Some(CellType::NumberCell(HintType::Some)), false)),
            _ => cells.push((None, false)),
        }
    }
    cells
}
