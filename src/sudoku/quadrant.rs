use crate::{SNumber};
use error_mapper::{create_new_error, TheResult};
use std::collections::HashSet;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub(crate) struct Quadrant {
    pub(crate) c1: SNumber,
    pub(crate) c2: SNumber,
    pub(crate) c3: SNumber,
    pub(crate) c4: SNumber,
    pub(crate) c5: SNumber,
    pub(crate) c6: SNumber,
    pub(crate) c7: SNumber,
    pub(crate) c8: SNumber,
    pub(crate) c9: SNumber,
}

impl Quadrant {
    pub(super) fn new(input: Vec<SNumber>) -> TheResult<Self> {
        //  Validate input length
        Self::validate_input_length(&input)?;

        //  Validate numbers for input array
        Self::validate_input_content(&input)?;

        //  All good, return mapped quad
        Ok(Self::map_input_to_quadrant(input))
    }

    fn validate_input_length(input: &[SNumber]) -> TheResult<()> {
        if input.len() > 9 {
            return Err(create_new_error!(
                "Input array length for a quadrant cannot be greater than 9!"
            ));
        }

        if input.len() < 9 {
            return Err(create_new_error!(
                "Input array length for a quadrant cannot be smaller than 9!"
            ));
        }

        Ok(())
    }

    fn validate_input_content(input: &[SNumber]) -> TheResult<()> {
        if !input.iter().all(|num| *num <= 9) {
            return Err(create_new_error!(
                "Input contained numbers out of Sudoku range"
            ));
        }

        //  Check for duplicates
        let mut set = HashSet::new();
        for item in input {
            if *item != 0 && !set.insert(*item) {
                return Err(create_new_error!(
                    "There were duplicate numbers in this input"
                ));
            }
        }

        Ok(())
    }

    fn map_input_to_quadrant(input: Vec<SNumber>) -> Self {
        Self {
            c1: input[0],
            c2: input[1],
            c3: input[2],
            c4: input[3],
            c5: input[4],
            c6: input[5],
            c7: input[6],
            c8: input[7],
            c9: input[8],
        }
    }

    fn is_number_available_in_quad(self, num: SNumber) -> bool {
        if num == 0 {
            return true;
        }

        self.c1 != num
            && self.c2 != num
            && self.c3 != num
            && self.c4 != num
            && self.c5 != num
            && self.c6 != num
            && self.c7 != num
            && self.c8 != num
            && self.c9 != num
    }

    pub(crate) fn get_row_array(&self, row_index: SNumber) -> TheResult<Vec<SNumber>> {
        match row_index {
            1 => Ok(vec![self.c1, self.c2, self.c3]),
            2 => Ok(vec![self.c4, self.c5, self.c6]),
            3 => Ok(vec![self.c7, self.c8, self.c9]),
            _ => Err(create_new_error!(format!("Index: {} out of bounds!", row_index))),
        }
    }

    pub(crate) fn get_col_array(&self, col_index: SNumber) -> TheResult<Vec<SNumber>> {
        match col_index {
            1 => Ok(vec![self.c1, self.c4, self.c7]),
            2 => Ok(vec![self.c2, self.c5, self.c8]),
            3 => Ok(vec![self.c3, self.c6, self.c9]),
            _ => Err(create_new_error!(format!("Index: {} out of bounds!", col_index))),
        }
    }
}

/////////////////////////////////////
//  Tests
/////////////////////////////////////
#[cfg(test)]
mod quad_tests {
    use crate::sudoku::quadrant::Quadrant;

    #[test]
    fn validate_input_length_ok() {
        let input = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];

        assert_eq!(Quadrant::validate_input_length(&input).unwrap(), ());
    }
    #[test]
    fn validate_input_length_err_length_longer() {
        let input = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 5];

        assert_eq!(
            Quadrant::validate_input_length(&input)
                .unwrap_err()
                .error
                .error_content,
            "Input array length for a quadrant cannot be greater than 9!"
        );
    }

    #[test]
    fn validate_input_length_length_shorter() {
        let input = vec![9, 8, 7, 6, 5, 4, 3, 2];

        assert_eq!(
            Quadrant::validate_input_length(&input)
                .unwrap_err()
                .error
                .error_content,
            "Input array length for a quadrant cannot be smaller than 9!"
        );
    }

    #[test]
    fn validate_input_content_ok() {
        let input = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

        //  Validation ok
        assert_eq!(Quadrant::validate_input_content(&input).unwrap(), ())
    }

    #[test]
    fn validate_input_content_ok_contains_zeroes() {
        let input = vec![1, 0, 3, 4, 5, 0, 7, 8, 9];

        //  Validation ok
        assert_eq!(Quadrant::validate_input_content(&input).unwrap(), ())
    }

    #[test]
    fn validate_input_content_err_bigger_number() {
        let input = vec![1, 2, 3, 4, 5, 15, 7, 8, 9];

        //  Validation ok
        assert_eq!(
            Quadrant::validate_input_content(&input)
                .unwrap_err()
                .error
                .error_content,
            "Input contained numbers out of Sudoku range"
        )
    }

    #[test]
    fn validate_input_content_err_duplicates() {
        let input = vec![1, 2, 3, 4, 5, 5, 7, 8, 9];

        //  Validation ok
        assert_eq!(
            Quadrant::validate_input_content(&input)
                .unwrap_err()
                .error
                .error_content,
            "There were duplicate numbers in this input"
        )
    }

    #[test]
    fn map_input_to_quad_ok() {
        let input = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];

        let expected = Quadrant {
            c1: 9,
            c2: 8,
            c3: 7,
            c4: 6,
            c5: 5,
            c6: 4,
            c7: 3,
            c8: 2,
            c9: 1,
        };

        assert_eq!(Quadrant::map_input_to_quadrant(input), expected);
    }

    #[test]
    fn new_quad_ok() {
        let input = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];

        let expected = Quadrant {
            c1: 9,
            c2: 8,
            c3: 7,
            c4: 6,
            c5: 5,
            c6: 4,
            c7: 3,
            c8: 2,
            c9: 1,
        };

        assert_eq!(Quadrant::new(input).unwrap(), expected);
    }

    #[test]
    fn new_quad_err_length_longer() {
        let input = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 5];

        assert_eq!(
            Quadrant::new(input).unwrap_err().error.error_content,
            "Input array length for a quadrant cannot be greater than 9!"
        );
    }

    #[test]
    fn new_quad_err_length_shorter() {
        let input = vec![9, 8, 7, 6, 5, 4, 3, 2];

        assert_eq!(
            Quadrant::new(input).unwrap_err().error.error_content,
            "Input array length for a quadrant cannot be smaller than 9!"
        );
    }

    #[test]
    fn new_quad_err_invalid_number() {
        let input = vec![9, 8, 7, 6, 5, 4, 3, 12, 1];

        assert_eq!(
            Quadrant::new(input).unwrap_err().error.error_content,
            "Input contained numbers out of Sudoku range"
        );
    }

    #[test]
    fn new_quad_err_duplicate_numbers() {
        let input = vec![9, 8, 1, 6, 5, 4, 3, 2, 1];

        assert_eq!(
            Quadrant::new(input).unwrap_err().error.error_content,
            "There were duplicate numbers in this input"
        );
    }

    #[test]
    fn is_number_available_ok_not_available() {
        let input = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let quad = Quadrant::new(input).unwrap();

        assert_eq!(quad.is_number_available_in_quad(7), false);
    }

    #[test]
    fn is_number_available_ok_is_available() {
        let input = vec![9, 0, 7, 0, 5, 4, 3, 0, 1];
        let quad = Quadrant::new(input).unwrap();

        assert_eq!(quad.is_number_available_in_quad(8), true);
    }

    #[test]
    fn is_number_available_ok_is_zero() {
        let input = vec![9, 0, 7, 0, 5, 4, 3, 0, 1];
        let quad = Quadrant::new(input).unwrap();

        assert_eq!(quad.is_number_available_in_quad(0), true);
    }

    #[test]
    fn get_row_array_ok_1() {
        let input = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let quad = Quadrant::new(input).unwrap();

        assert_eq!(quad.get_row_array(1).unwrap(), vec![9, 8, 7]);
    }

    #[test]
    fn get_row_array_ok_2() {
        let input = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let quad = Quadrant::new(input).unwrap();

        assert_eq!(quad.get_row_array(2).unwrap(), vec![6, 5, 4]);
    }

    #[test]
    fn get_row_array_ok_3() {
        let input = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let quad = Quadrant::new(input).unwrap();

        assert_eq!(quad.get_row_array(3).unwrap(), vec![3, 2, 1]);
    }

    #[test]
    fn get_row_array_err() {
        let input = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let quad = Quadrant::new(input).unwrap();

        assert_eq!(
            quad.get_row_array(5).unwrap_err().error.error_content,
            "Index: 5 out of bounds!"
        );
    }

    #[test]
    fn get_col_array_ok_1() {
        let input = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let quad = Quadrant::new(input).unwrap();

        assert_eq!(quad.get_col_array(1).unwrap(), vec![9, 6, 3]);
    }

    #[test]
    fn get_col_array_ok_2() {
        let input = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let quad = Quadrant::new(input).unwrap();

        assert_eq!(quad.get_col_array(2).unwrap(), vec![8, 5, 2]);
    }

    #[test]
    fn get_col_array_ok_3() {
        let input = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let quad = Quadrant::new(input).unwrap();

        assert_eq!(quad.get_col_array(3).unwrap(), vec![7, 4, 1]);
    }

    #[test]
    fn get_col_array_err() {
        let input = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
        let quad = Quadrant::new(input).unwrap();

        assert_eq!(
            quad.get_col_array(5).unwrap_err().error.error_content,
            "Index: 5 out of bounds!"
        );
    }
}
