use std::panic::panic_any;
use crate::sudoku::Sudoku;

mod sudoku;

type SNumber = u8;
type InputNumber = i32;
type QuadVector = Vec<SNumber>;

//  TODO further define the return type for errors
pub fn solve_sudoku_from_array(input: Vec<InputNumber>) -> Vec<SNumber> {
    
    let _sudoku = match Sudoku::new(input) {
        Ok(sudoku) => sudoku,
        Err(e) => {
            panic_any(e.to_string())
        }
    };
    
    vec![]
}
