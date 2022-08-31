use crate::components::{CellType, HintType};

pub fn board_to_string(cells: &[Vec<(Option<CellType>, bool)>]) -> String {
    format!(
        "{},{}\n{}",
        cells[0].len(),
        cells.len(),
        cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|entry| match *entry {
                        (Some(CellType::EmptyCell), true) => '0',
                        (Some(CellType::EmptyCell), false) => '1',
                        (Some(CellType::NumberCell(HintType::None)), true) => '2',
                        (Some(CellType::NumberCell(HintType::None)), false) => '3',
                        (Some(CellType::NumberCell(_)), true) => '4',
                        (Some(CellType::NumberCell(_)), false) => '5',
                        (None, _) => '.',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    )
}
