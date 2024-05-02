use crate::{QuadVector, SNumber};
use error_mapper::{create_new_error, TheResult};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub(super) struct Quadrant {
    c1: SNumber,
    c2: SNumber,
    c3: SNumber,
    c4: SNumber,
    c5: SNumber,
    c6: SNumber,
    c7: SNumber,
    c8: SNumber,
    c9: SNumber,
}

impl Quadrant {
    pub(super) fn new(input: QuadVector) -> TheResult<Self> {
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
            return Err(create_new_error!("Input contained numbers out of Sudoku range"))
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
        assert_eq!(Quadrant::validate_input_content(&input).unwrap_err().error.error_content, "Input contained numbers out of Sudoku range")
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
}
