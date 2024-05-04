use crate::sudoku::Sudoku;
use std::panic::panic_any;

mod sudoku;

type SNumber = u8;
type InputNumber = i32;

//  TODO further define the return type for errors
pub fn solve_sudoku_from_array(input: Vec<InputNumber>) -> Vec<SNumber> {
    let sudoku = match Sudoku::new(input) {
        Ok(sudoku) => sudoku,
        Err(e) => panic_any(e.to_string()),
    };

    vec![]
}
