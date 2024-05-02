use crate::sudoku::quadrant::Quadrant;
use crate::{InputNumber, QuadVector, SNumber};
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
    pub(super) fn new(input: Vec<InputNumber>) -> TheResult<Self> {

        //  Validate input length
        Self::validate_input_length(&input)?;

        //  Validate input content
        Self::validate_input_content(&input)?;
        
        //  Map input to u8 vector
        let mut mapped_input = Self::map_input_to_u8(input);

        Ok(Self::build_quads_from_mapped_input(&mut mapped_input)?)
    }

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

    fn validate_input_content(input: &[InputNumber]) -> TheResult<()> {
        if !input.iter().all(|num| *num <= 9 && *num >= 0) {
            return Err(create_new_error!("Input contained numbers out of Sudoku range"))
        }

        Ok(())
    }

    fn map_input_to_u8(input: Vec<InputNumber>) -> Vec<SNumber> {
        input
            .into_iter()
            .map(|num| num as SNumber)
            .collect::<Vec<SNumber>>()
    }
    
    fn build_quads_from_mapped_input(input: &mut Vec<SNumber>) -> TheResult<Self> {
        
        //  Build all 9 quads from the mapped input vector
        let quad_1 = Quadrant::new(Self::get_slice_for_quad(input)?)?;
        let quad_2 = Quadrant::new(Self::get_slice_for_quad(input)?)?;
        let quad_3 = Quadrant::new(Self::get_slice_for_quad(input)?)?;
        let quad_4 = Quadrant::new(Self::get_slice_for_quad(input)?)?;
        let quad_5 = Quadrant::new(Self::get_slice_for_quad(input)?)?;
        let quad_6 = Quadrant::new(Self::get_slice_for_quad(input)?)?;
        let quad_7 = Quadrant::new(Self::get_slice_for_quad(input)?)?;
        let quad_8 = Quadrant::new(Self::get_slice_for_quad(input)?)?;
        let quad_9 = Quadrant::new(Self::get_slice_for_quad(input)?)?;
        
        Ok(Self {
            q1: quad_1,
            q2: quad_2,
            q3: quad_3,
            q4: quad_4,
            q5: quad_5,
            q6: quad_6,
            q7: quad_7,
            q8: quad_8,
            q9: quad_9,
        })
    }

    fn get_slice_for_quad(input: &mut Vec<SNumber>) -> TheResult<QuadVector> {
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
        assert_eq!(Sudoku::validate_input_content(&input).unwrap(), ())
    }

    #[test]
    fn validate_input_content_ok_contains_zeroes() {
        let input = vec![1, 2, 3, 4, 5, 0, 7, 8, 9, 1, 0, 3, 4, 5, 6, 0, 8, 9];

        //  Validation ok
        assert_eq!(Sudoku::validate_input_content(&input).unwrap(), ())
    }

    #[test]
    fn validate_input_content_err_bigger_number() {
        let input = vec![1, 2, 3, 4, 5, 15, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        //  Validation ok
        assert_eq!(Sudoku::validate_input_content(&input).unwrap_err().error.error_content, "Input contained numbers out of Sudoku range")
    }

    #[test]
    fn validate_input_content_err_negative_number() {
        let input = vec![1, 2, 3, 4, 5, -15, 7, 8, 9, 1, 2, 3, 4, -5, 6, 7, 8, 9];

        //  Validation ok
        assert_eq!(Sudoku::validate_input_content(&input).unwrap_err().error.error_content, "Input contained numbers out of Sudoku range")
    }

    #[test]
    fn get_slice_for_quad_ok_nine_elements() {
        let mut input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        let quad_slice = Sudoku::get_slice_for_quad(&mut input).unwrap();

        //  Should convert ok and drain the input vector
        assert_eq!(input, vec![]);
        assert_eq!(quad_slice, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn get_slice_for_quad_ok_over_nine_elements() {
        let mut input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4];

        let quad_slice = Sudoku::get_slice_for_quad(&mut input).unwrap();

        //  Should convert ok and leave the input vector with 4 elements only
        assert_eq!(input, vec![1, 2, 3, 4]);
        assert_eq!(quad_slice, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn get_slice_for_quad_err() {
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
