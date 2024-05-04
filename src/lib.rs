use std::panic::panic_any;
use error_mapper::{create_new_error, TheResult};

type SNumber = u8;
type InputNumber = i32;
type SudokuArray = Vec<Vec<SNumber>>;

pub fn solve_sudoku(sudoku: Vec<InputNumber>) {

    let mut sudoku = match init_input(sudoku) {
        Ok(board) => board,
        Err(e) => {
            panic_any(e.to_string());
        }
    };

    if solve(&mut sudoku) {
        println!("Sudoku solved!");
    } else {
        println!("Sudoku had no solution!");
    }
    print_sudoku(&sudoku);

}

fn init_input(mut input_vector: Vec<InputNumber>) -> TheResult<SudokuArray> {

    //  Build sudoku from input
    let sudoku = parse_from_input(&mut input_vector)?;

    //  Validate locations in case they're wrong
    validate_input_locations(&sudoku)?;

    Ok(sudoku)
}

fn parse_from_input(input: &mut Vec<InputNumber>) -> TheResult<SudokuArray> {

    let mut sudoku = vec![];

    //  Validate no numbers are negative or greater than 9
    if !input.iter().all(|num| *num <= 9 && *num >= 0) {
        return Err(create_new_error!("There are invalid numbers in input vector!"))
    }

    //  Drain input vector to build sudoku board
    for _ in 0..9 {
        sudoku.push(input.drain(0..9).into_iter().map(|num| num as SNumber).collect::<Vec<SNumber>>());
    }

    Ok(sudoku)
}

fn validate_input_locations(sudoku: &SudokuArray) -> TheResult<()> {

    for row in 0..9 {
        for col in 0..9 {
            if sudoku[row][col] == 0 {
                continue;
            }
            let mut temp_sudoku = sudoku.clone();
            temp_sudoku[row][col] = 0;
            if !is_number_valid(&temp_sudoku, sudoku[row][col], (row, col)) {
                return Err(create_new_error!(format!("Invalid cell number: {} at (row {} / col {})", sudoku[row][col], row, col)))
            }
        }
    }

    Ok(())
}

fn find_next_empty_cell(sudoku: &Vec<Vec<SNumber>>) -> Option<(usize, usize)> {
    for (row, row_data) in sudoku.iter().enumerate() {
        for (col, cell) in row_data.iter().enumerate() {
            if *cell == 0 {
                return Some((row, col))
            }
        }
    }

    None
}

fn is_number_valid(sudoku: &Vec<Vec<SNumber>>, number: SNumber, (row, col): (usize, usize)) -> bool {

    //  Check validity in row
    if sudoku[row].contains(&number) {
        return false
    }

    //  Check validity in column
    for index in 0..9 {
        if sudoku[index][col] == number {
            return false
        }
    }

    //  Check validity in quadrant
    let row_start_index = row / 3;
    let col_start_index = col / 3;

    for row_index in 0..3 {
        for col_index in 0..3 {
            let row_location = row_start_index * 3 + row_index;
            let col_location = col_start_index * 3 + col_index;

            if sudoku[row_location][col_location] == number {
                return false
            }
        }
    }

    //  Number is valid, proceed
    true
}

fn solve(sudoku: &mut SudokuArray) -> bool {

    //  Find next empty cell
    if let Some((cell_row, cell_col)) = find_next_empty_cell(sudoku) {
        //  If there's an empty cell, iterate through numbers 1 through 9 to find the valid one
        for number in 1..=9 {

            if is_number_valid(sudoku, number, (cell_row, cell_col)) {
                sudoku[cell_row][cell_col] = number;
                if solve(sudoku) {
                    return true
                }
                sudoku[cell_row][cell_col] = 0;
            }
        }
        return false
    } else {
        //  If there's no empty cells then return
        true
    }
}

fn print_sudoku(sudoku: &SudokuArray) {
    for row in sudoku {
        for col in row {
            print!("{} ", col);
        }
        println!(" ");
    }
}

#[cfg(test)]
mod test {
    use crate::{InputNumber, solve_sudoku};

    #[test]
    fn test_main() {

        solve_sudoku(input_3());

        assert!(false);
    }

    fn input_1() -> Vec<InputNumber> {
        vec![
            5,3,0,0,7,0,0,0,0,
            6,0,0,1,9,5,0,0,0,
            0,9,8,0,0,0,0,6,0,
            8,0,0,0,6,0,0,0,3,
            4,0,0,8,0,3,0,0,1,
            7,0,0,0,2,0,0,0,6,
            0,6,0,0,0,0,2,8,0,
            0,0,0,4,1,9,0,0,5,
            0,0,0,0,8,0,0,7,9
        ]
    }
    
    fn input_2() -> Vec<InputNumber> {
        vec![
            5,3,0,0,7,0,0,0,0,
            6,0,0,1,9,5,0,0,0,
            0,9,8,0,0,0,0,6,0,
            8,0,0,0,6,0,0,0,3,
            4,0,0,8,0,3,0,0,1,
            7,0,0,0,2,0,0,0,6,
            0,6,0,0,0,0,2,8,0,
            0,0,0,4,1,9,0,0,5,
            0,0,0,0,8,0,0,7,9
        ]
    }
    
    fn input_3() -> Vec<InputNumber> {
        vec![
            5,3,0,0,7,0,0,0,0,
            6,0,0,1,9,5,0,0,0,
            0,9,8,0,0,0,0,6,0,
            8,0,0,0,6,0,0,0,3,
            4,0,0,8,0,3,0,0,1,
            7,0,0,0,2,0,0,0,6,
            0,6,0,0,0,0,2,8,0,
            0,0,0,4,1,9,0,0,5,
            0,0,0,0,8,0,0,7,9
        ]
    }
}
