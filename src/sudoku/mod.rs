use crate::sudoku::quadrant::Quadrant;
use crate::{InputNumber, SNumber};
use error_mapper::{create_new_error, TheResult};

mod quadrant;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Sudoku {
    q1: Quadrant,
    q2: Quadrant,
    q3: Quadrant,
    q4: Quadrant,
    q5: Quadrant,
    q6: Quadrant,
    q7: Quadrant,
    q8: Quadrant,
    q9: Quadrant,
}

impl Sudoku {
    pub(super) fn new(input: Vec<InputNumber>) {}

    fn validate_input_length(input: &[InputNumber]) -> TheResult<()> {
        if input.len() > 81 {
            return Err(create_new_error!(
                "Input array length for a quadrant cannot be greater than 81!"
            ));
        }

        if input.len() < 81 {
            return Err(create_new_error!(
                "Input array length for a quadrant cannot be smaller than 81!"
            ));
        }

        Ok(())
    }

    fn validate_input_content(input: &[InputNumber]) -> bool {
        input.iter().all(|num| *num <= 9 && *num >= 0)
    }

    fn map_input_to_u8(input: Vec<InputNumber>) -> Vec<SNumber> {
        input
            .into_iter()
            .map(|num| num as SNumber)
            .collect::<Vec<SNumber>>()
    }

    fn get_slice_for_quad(input: &mut Vec<SNumber>) -> TheResult<Vec<SNumber>> {
        if input.len() < 9 {
            return Err(create_new_error!(
                "Insufficient input length to obtain a slice for a quadrant"
            ));
        }

        Ok(input.drain(0..9).collect())
    }
}

#[cfg(test)]
mod sudoku_tests {
    use crate::sudoku::Sudoku;

    #[test]
    fn validate_input_length_ok() {
        let input = vec![1; 81];

        assert_eq!(Sudoku::validate_input_length(&input).unwrap(), ());
    }

    #[test]
    fn validate_input_length_err_longer() {
        let input = vec![1; 87];

        assert_eq!(
            Sudoku::validate_input_length(&input)
                .unwrap_err()
                .error
                .error_content,
            "Input array length for a quadrant cannot be greater than 81!"
        );
    }

    #[test]
    fn validate_input_length_err_shorter() {
        let input = vec![1; 70];

        assert_eq!(
            Sudoku::validate_input_length(&input)
                .unwrap_err()
                .error
                .error_content,
            "Input array length for a quadrant cannot be smaller than 81!"
        );
    }

    #[test]
    fn validate_input_content_ok() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        //  Validation ok
        assert_eq!(Sudoku::validate_input_content(&input), true)
    }

    #[test]
    fn validate_input_content_ok_contains_zeroes() {
        let input = vec![1, 2, 3, 4, 5, 0, 7, 8, 9, 1, 0, 3, 4, 5, 6, 0, 8, 9];

        //  Validation ok
        assert_eq!(Sudoku::validate_input_content(&input), true)
    }

    #[test]
    fn validate_input_content_err_bigger_number() {
        let input = vec![1, 2, 3, 4, 5, 15, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        //  Validation ok
        assert_eq!(Sudoku::validate_input_content(&input), false)
    }

    #[test]
    fn validate_input_content_err_negative_number() {
        let input = vec![1, 2, 3, 4, 5, -15, 7, 8, 9, 1, 2, 3, 4, -5, 6, 7, 8, 9];

        //  Validation ok
        assert_eq!(Sudoku::validate_input_content(&input), false)
    }

    #[test]
    fn test_get_slice_for_quad_ok_nine_elements() {
        let mut input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let quad_slice = Sudoku::get_slice_for_quad(&mut input).unwrap();

        //  Should convert ok and drain the input vector
        assert_eq!(input, vec![]);
        assert_eq!(quad_slice, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_get_slice_for_quad_ok_over_nine_elements() {
        let mut input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4];

        let quad_slice = Sudoku::get_slice_for_quad(&mut input).unwrap();

        //  Should convert ok and leave the input vector with 4 elements only
        assert_eq!(input, vec![1, 2, 3, 4]);
        assert_eq!(quad_slice, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_get_slice_for_quad_err() {
        let mut input = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let result = Sudoku::get_slice_for_quad(&mut input).unwrap_err();

        //  Should return error because of the length and not touch the input vector
        assert_eq!(input, vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(
            result.error.error_content,
            "Insufficient input length to obtain a slice for a quadrant"
        );
    }
}
