use crate::enums::{CellType, HintDirection};
use std::collections::VecDeque;

/// Get a ordered list of neighbouring cells
pub fn get_neighbours(
    x: i32,
    y: i32,
    cells: &[Vec<(Option<CellType>, bool)>],
    w: usize,
    h: usize,
) -> Vec<(Option<CellType>, bool)> {
    let pos = if x % 2 == 0 {
        [
            (x, y - 1),
            (x + 1, y),
            (x + 1, y + 1),
            (x, y + 1),
            (x - 1, y + 1),
            (x - 1, y),
        ]
    } else {
        [
            (x, y - 1),
            (x + 1, y - 1),
            (x + 1, y),
            (x, y + 1),
            (x - 1, y),
            (x - 1, y - 1),
        ]
    };
    pos.iter()
        .map(|(x, y)| {
            if *x < 0 || *x >= w as i32 || *y < 0 || *y >= h as i32 {
                (None, false)
            } else {
                cells[*y as usize][*x as usize]
            }
        })
        .collect()
}

/// Get a ordered list of cells in same column (or diagonal)
pub fn get_column(
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    cells: &[Vec<(Option<CellType>, bool)>],
    dir: HintDirection,
) -> Vec<(Option<CellType>, bool)> {
    match dir {
        HintDirection::Down => (y..h).into_iter().map(|dy| cells[dy][x]).collect(),
        HintDirection::LeftDown => {
            let mut pts = VecDeque::new();
            let mut dx = x;
            let mut dy = y;
            while dx > 0 && (dy < h - 1 || dx % 2 == 1) {
                if dx % 2 == 0 {
                    dy += 1;
                }
                dx -= 1;
                pts.push_front(cells[dy][dx]);
            }
            pts.push_back(cells[y][x]);
            pts.into()
        }
        HintDirection::RightDown => {
            let mut pts = VecDeque::new();
            pts.push_back(cells[y][x]);
            let mut dx = x;
            let mut dy = y;
            while dx < w - 1 && (dy < h - 1 || dx % 2 == 1) {
                if dx % 2 == 0 {
                    dy += 1;
                }
                dx += 1;
                pts.push_back(cells[dy][dx]);
            }
            pts.into()
        }
        HintDirection::Up => (0..=y).into_iter().map(|dy| cells[dy][x]).collect(),
        HintDirection::LeftUp => {
            let mut pts = VecDeque::new();
            let mut dx = x;
            let mut dy = y;
            while dx > 0 && (dy > 0 || dx % 2 == 0) {
                if dx % 2 == 1 {
                    dy -= 1;
                }
                dx -= 1;
                pts.push_front(cells[dy][dx]);
            }
            pts.push_back(cells[y][x]);
            pts.into()
        }
        HintDirection::RightUp => {
            let mut pts = VecDeque::new();
            pts.push_back(cells[y][x]);
            let mut dx = x;
            let mut dy = y;
            while dx < w - 1 && (dy > 0 || dx % 2 == 0) {
                if dx % 2 == 1 {
                    dy -= 1;
                }
                dx += 1;
                pts.push_back(cells[dy][dx]);
            }
            pts.into()
        }
    }
}

/// Count how many cells in a list are empty
pub fn count_empty_cells(cells: &[(Option<CellType>, bool)]) -> u8 {
    cells
        .iter()
        .map(|(c, _)| {
            if let Some(ct) = c {
                if *ct == CellType::EmptyCell {
                    1
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum()
}

/// Check if the empty cells are connected or seperated
pub fn empty_connected(cells: &[(Option<CellType>, bool)], count: u8, circular: bool) -> bool {
    if count <= 1 {
        return true;
    }
    // TODO: So many clones...
    let mut cells = cells.to_owned();
    if circular {
        cells.extend(cells.clone());
    }
    let mut second_chance = circular;
    let mut remaining = count;
    let mut begun = false;
    for (ct, _) in cells {
        if remaining == 0 {
            return true;
        }
        if begun {
            if let Some(ct) = ct {
                if ct == CellType::EmptyCell {
                    remaining -= 1;
                } else if second_chance {
                    second_chance = false;
                    remaining = count;
                    begun = false;
                } else {
                    break;
                }
            } else if second_chance {
                second_chance = false;
                remaining = count;
                begun = false;
            } else if circular {
                break;
            }
        } else if let Some(ct) = ct {
            if ct == CellType::EmptyCell {
                begun = true;
                remaining -= 1;
            }
        }
    }
    if remaining == 0 {
        return true;
    }
    false
}
