use crate::components::{CellType, HintDirection};
use std::collections::VecDeque;

/// Get a (ordered?) list of neighbouring cells
pub fn get_neighbours(
    x: i32,
    y: i32,
    cells: &[Vec<Option<CellType>>],
    w: usize,
    h: usize,
) -> Vec<Option<CellType>> {
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
        .filter(|(x, y)| !(*x < 0 || *x >= w as i32 || *y < 0 || *y >= h as i32))
        .map(|(x, y)| (cells[*y as usize][*x as usize]))
        .collect()
}

/// Get a (ordered?) list of cells in same column (or diagonal)
pub fn get_column(
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    cells: &[Vec<Option<CellType>>],
    dir: HintDirection,
) -> Vec<Option<CellType>> {
    match dir {
        HintDirection::Top => (0..h).into_iter().map(|dy| cells[dy][x]).collect(),
        HintDirection::Left => {
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
        HintDirection::Right => {
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
pub fn count_empty_cells(cells: &[Option<CellType>]) -> u8 {
    cells
        .iter()
        .map(|c| {
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
pub fn empty_connected(cells: &[Option<CellType>], count: u8, circular: bool) -> bool {
    if count == 0 {
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
    for ct in cells {
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
            }
        } else if let Some(ct) = ct {
            if ct == CellType::EmptyCell {
                begun = true;
                remaining -= 1;
            }
        }
    }
    false
}
