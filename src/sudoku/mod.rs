use crate::sudoku::quadrant::Quadrant;
use crate::{InputNumber, SNumber};
use error_mapper::{create_new_error, TheResult};
use std::collections::HashSet;

mod quadrant;

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct Sudoku {
    pub(crate) q1: Quadrant,
    pub(crate) q2: Quadrant,
    pub(crate) q3: Quadrant,
    pub(crate) q4: Quadrant,
    pub(crate) q5: Quadrant,
    pub(crate) q6: Quadrant,
    pub(crate) q7: Quadrant,
    pub(crate) q8: Quadrant,
    pub(crate) q9: Quadrant,
}

impl Sudoku {
    pub(super) fn new(input: Vec<InputNumber>) -> TheResult<Self> {
        //  Validate input length
        Self::validate_input_length(&input)?;

        //  Validate input content
        Self::validate_input_content(&input)?;

        //  Map input to u8 vector
        let mut mapped_input = Self::map_input_to_u8(input);

        //  Build sudoku struct
        let sudoku = Self::parse_input_into_sudoku(&mut mapped_input)?;

        //  Validate there are no duplicate values in rows and columns
        sudoku.validate_no_duplicates_in_row_and_col()?;

        Ok(sudoku)
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
            return Err(create_new_error!(
                "Input contained numbers out of Sudoku range"
            ));
        }

        Ok(())
    }

    fn map_input_to_u8(input: Vec<InputNumber>) -> Vec<SNumber> {
        input
            .into_iter()
            .map(|num| num as SNumber)
            .collect::<Vec<SNumber>>()
    }

    //  Allow identity op allows to not suggest deleting the 0 + ... when parsing the original array
    #[allow(clippy::identity_op)]
    fn parse_input_into_sudoku(input: &mut Vec<SNumber>) -> TheResult<Self> {
        if input.len() < 9 {
            return Err(create_new_error!(
                "Insufficient input length to obtain a slice for a quadrant"
            ));
        }

        let mut temp = vec![];
        //  Quads 1, 2 and 3
        for outer in 0..3 {
            for inner in 0..3 {
                temp.push(input[0 + (inner * 9) + (outer * 3)]);
                temp.push(input[1 + (inner * 9) + (outer * 3)]);
                temp.push(input[2 + (inner * 9) + (outer * 3)]);
            }
        }

        //  Quads 4, 5 and 6
        for outer in 0..3 {
            for inner in 0..3 {
                temp.push(input[27 + (inner * 9) + (outer * 3)]);
                temp.push(input[28 + (inner * 9) + (outer * 3)]);
                temp.push(input[29 + (inner * 9) + (outer * 3)]);
            }
        }

        //  Quads 7, 8 and 9
        for outer in 0..3 {
            for inner in 0..3 {
                temp.push(input[54 + (inner * 9) + (outer * 3)]);
                temp.push(input[55 + (inner * 9) + (outer * 3)]);
                temp.push(input[56 + (inner * 9) + (outer * 3)]);
            }
        }

        let quad_1 = Quadrant::new(temp.drain(0..9).collect())?;
        let quad_2 = Quadrant::new(temp.drain(0..9).collect())?;
        let quad_3 = Quadrant::new(temp.drain(0..9).collect())?;
        let quad_4 = Quadrant::new(temp.drain(0..9).collect())?;
        let quad_5 = Quadrant::new(temp.drain(0..9).collect())?;
        let quad_6 = Quadrant::new(temp.drain(0..9).collect())?;
        let quad_7 = Quadrant::new(temp.drain(0..9).collect())?;
        let quad_8 = Quadrant::new(temp.drain(0..9).collect())?;
        let quad_9 = Quadrant::new(temp.drain(0..9).collect())?;

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

    fn get_row_array(&self, row_index: SNumber) -> TheResult<Vec<SNumber>> {
        match row_index {
            1 => {
                let mut vec = self.q1.get_row_array(1)?;
                vec.extend(self.q2.get_row_array(1)?);
                vec.extend(self.q3.get_row_array(1)?);
                Ok(vec)
            }
            2 => {
                let mut vec = self.q1.get_row_array(2)?;
                vec.extend(self.q2.get_row_array(2)?);
                vec.extend(self.q3.get_row_array(2)?);
                Ok(vec)
            }
            3 => {
                let mut vec = self.q1.get_row_array(3)?;
                vec.extend(self.q2.get_row_array(3)?);
                vec.extend(self.q3.get_row_array(3)?);
                Ok(vec)
            }
            4 => {
                let mut vec = self.q4.get_row_array(1)?;
                vec.extend(self.q5.get_row_array(1)?);
                vec.extend(self.q6.get_row_array(1)?);
                Ok(vec)
            }
            5 => {
                let mut vec = self.q4.get_row_array(2)?;
                vec.extend(self.q5.get_row_array(2)?);
                vec.extend(self.q6.get_row_array(2)?);
                Ok(vec)
            }
            6 => {
                let mut vec = self.q4.get_row_array(3)?;
                vec.extend(self.q5.get_row_array(3)?);
                vec.extend(self.q6.get_row_array(3)?);
                Ok(vec)
            }
            7 => {
                let mut vec = self.q7.get_row_array(1)?;
                vec.extend(self.q8.get_row_array(1)?);
                vec.extend(self.q9.get_row_array(1)?);
                Ok(vec)
            }
            8 => {
                let mut vec = self.q7.get_row_array(2)?;
                vec.extend(self.q8.get_row_array(2)?);
                vec.extend(self.q9.get_row_array(2)?);
                Ok(vec)
            }
            9 => {
                let mut vec = self.q7.get_row_array(3)?;
                vec.extend(self.q8.get_row_array(3)?);
                vec.extend(self.q9.get_row_array(3)?);
                Ok(vec)
            }
            _ => Err(create_new_error!("Index out of bounds!")),
        }
    }

    fn get_col_array(&self, col_index: SNumber) -> TheResult<Vec<SNumber>> {
        match col_index {
            1 => {
                let mut vec = self.q1.get_col_array(1)?;
                vec.extend(self.q4.get_col_array(1)?);
                vec.extend(self.q7.get_col_array(1)?);
                Ok(vec)
            }
            2 => {
                let mut vec = self.q1.get_col_array(2)?;
                vec.extend(self.q4.get_col_array(2)?);
                vec.extend(self.q7.get_col_array(2)?);
                Ok(vec)
            }
            3 => {
                let mut vec = self.q1.get_col_array(3)?;
                vec.extend(self.q4.get_col_array(3)?);
                vec.extend(self.q7.get_col_array(3)?);
                Ok(vec)
            }
            4 => {
                let mut vec = self.q2.get_col_array(1)?;
                vec.extend(self.q5.get_col_array(1)?);
                vec.extend(self.q8.get_col_array(1)?);
                Ok(vec)
            }
            5 => {
                let mut vec = self.q2.get_col_array(2)?;
                vec.extend(self.q5.get_col_array(2)?);
                vec.extend(self.q8.get_col_array(2)?);
                Ok(vec)
            }
            6 => {
                let mut vec = self.q2.get_col_array(3)?;
                vec.extend(self.q5.get_col_array(3)?);
                vec.extend(self.q8.get_col_array(3)?);
                Ok(vec)
            }
            7 => {
                let mut vec = self.q3.get_col_array(1)?;
                vec.extend(self.q6.get_col_array(1)?);
                vec.extend(self.q9.get_col_array(1)?);
                Ok(vec)
            }
            8 => {
                let mut vec = self.q3.get_col_array(2)?;
                vec.extend(self.q6.get_col_array(2)?);
                vec.extend(self.q9.get_col_array(2)?);
                Ok(vec)
            }
            9 => {
                let mut vec = self.q3.get_col_array(3)?;
                vec.extend(self.q6.get_col_array(3)?);
                vec.extend(self.q9.get_col_array(3)?);
                Ok(vec)
            }
            _ => Err(create_new_error!("Index out of bounds!")),
        }
    }

    fn validate_no_duplicates_in_row_and_col(&self) -> TheResult<()> {
        for index in 1..10 {
            //  Check for duplicates in rows
            let vec = self.get_row_array(index)?;
            let mut set = HashSet::new();
            for item in vec {
                if item != 0 && !set.insert(item) {
                    return Err(create_new_error!(format!(
                        "Duplicate number in row {}",
                        index
                    )));
                }
            }
            set.clear();

            //  Check for duplicates in columns
            let vec = self.get_col_array(index)?;
            let mut set = HashSet::new();
            for item in vec {
                if item != 0 && !set.insert(item) {
                    return Err(create_new_error!(format!(
                        "Duplicate number in column {}",
                        index
                    )));
                }
            }
        }

        Ok(())
    }
}

/////////////////////////////////
//  Tests
/////////////////////////////////

#[cfg(test)]
mod sudoku_tests {
    use crate::sudoku::quadrant::Quadrant;
    use crate::sudoku::Sudoku;
    use crate::{InputNumber, SNumber};

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
        assert_eq!(
            Sudoku::validate_input_content(&input)
                .unwrap_err()
                .error
                .error_content,
            "Input contained numbers out of Sudoku range"
        )
    }

    #[test]
    fn validate_input_content_err_negative_number() {
        let input = vec![1, 2, 3, 4, 5, -15, 7, 8, 9, 1, 2, 3, 4, -5, 6, 7, 8, 9];

        //  Validation ok
        assert_eq!(
            Sudoku::validate_input_content(&input)
                .unwrap_err()
                .error
                .error_content,
            "Input contained numbers out of Sudoku range"
        )
    }

    #[test]
    fn new_sudoku_ok() {
        let input = build_sudoku_static();

        let expected_sudoku = build_sudoku_static_struct();

        assert_eq!(Sudoku::new(input).unwrap(), expected_sudoku);
    }

    #[test]
    fn new_sudoku_ok_contains_zeroes() {
        let input = build_sudoku_static_with_zeroes();

        let expected_sudoku = build_sudoku_static_struct_with_zeroes();

        assert_eq!(Sudoku::new(input).unwrap(), expected_sudoku);
    }

    #[test]
    fn new_sudoku_err_more_elements_than_allowed() {
        let mut input = vec![];
        for _ in 0..10 {
            let sub_slice = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            input.extend(sub_slice);
        }

        assert_eq!(
            Sudoku::new(input).unwrap_err().error.error_content,
            "Input array length for a quadrant cannot be greater than 81!"
        );
    }

    #[test]
    fn new_sudoku_err_fewer_elements_than_allowed() {
        let mut input = vec![];
        for _ in 0..8 {
            let sub_slice = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            input.extend(sub_slice);
        }

        assert_eq!(
            Sudoku::new(input).unwrap_err().error.error_content,
            "Input array length for a quadrant cannot be smaller than 81!"
        );
    }

    #[test]
    fn new_sudoku_err_numbers_bigger_than_9() {
        let mut input = vec![];
        for _ in 0..9 {
            let sub_slice = vec![1, 2, 3, 4, 15, 6, 7, 8, 9];
            input.extend(sub_slice);
        }

        assert_eq!(
            Sudoku::new(input).unwrap_err().error.error_content,
            "Input contained numbers out of Sudoku range"
        );
    }

    #[test]
    fn new_sudoku_err_negative_numbers() {
        let mut input = vec![];
        for _ in 0..9 {
            let sub_slice = vec![1, 2, -3, 4, 5, 6, -7, 8, 9];
            input.extend(sub_slice);
        }

        assert_eq!(
            Sudoku::new(input).unwrap_err().error.error_content,
            "Input contained numbers out of Sudoku range"
        );
    }

    #[test]
    fn get_row_array_ok_1() {
        let input = build_sudoku_static();

        let sudoku = Sudoku::new(input).unwrap();

        assert_eq!(
            sudoku.get_row_array(1).unwrap(),
            vec![8, 2, 7, 1, 5, 4, 3, 9, 6]
        );
    }

    #[test]
    fn get_row_array_ok_2() {
        let input = build_sudoku_static();

        let sudoku = Sudoku::new(input).unwrap();

        assert_eq!(
            sudoku.get_row_array(5).unwrap(),
            vec![4, 7, 2, 5, 1, 3, 6, 8, 9]
        );
    }

    #[test]
    fn get_row_array_ok_3() {
        let input = build_sudoku_static();

        let sudoku = Sudoku::new(input).unwrap();

        assert_eq!(
            sudoku.get_row_array(9).unwrap(),
            vec![2, 3, 9, 8, 4, 1, 5, 6, 7]
        );
    }

    #[test]
    fn get_row_array_err() {
        let input = build_sudoku_static();

        let sudoku = Sudoku::new(input).unwrap();

        assert_eq!(
            sudoku.get_row_array(10).unwrap_err().error.error_content,
            "Index out of bounds!"
        );
    }

    #[test]
    fn get_col_array_ok_1() {
        let input = build_sudoku_static();

        let sudoku = Sudoku::new(input).unwrap();

        assert_eq!(
            sudoku.get_col_array(1).unwrap(),
            vec![8, 9, 3, 5, 4, 6, 7, 1, 2]
        );
    }

    #[test]
    fn get_col_array_ok_2() {
        let input = build_sudoku_static();

        let sudoku = Sudoku::new(input).unwrap();

        assert_eq!(
            sudoku.get_col_array(5).unwrap(),
            vec![5, 2, 8, 6, 1, 7, 3, 9, 4]
        );
    }

    #[test]
    fn get_col_array_ok_3() {
        let input = build_sudoku_static();

        let sudoku = Sudoku::new(input).unwrap();

        assert_eq!(
            sudoku.get_col_array(9).unwrap(),
            vec![6, 8, 2, 1, 9, 5, 4, 3, 7]
        );
    }

    #[test]
    fn get_col_array_err() {
        let input = build_sudoku_static();

        let sudoku = Sudoku::new(input).unwrap();

        assert_eq!(
            sudoku.get_col_array(10).unwrap_err().error.error_content,
            "Index out of bounds!"
        );
    }

    #[test]
    fn parse_input_into_sudoku_ok() {
        let mut input = build_sudoku_static()
            .into_iter()
            .map(|item| item as SNumber)
            .collect::<Vec<SNumber>>();
        let parsed = Sudoku::parse_input_into_sudoku(&mut input);

        assert_eq!(parsed.unwrap(), build_sudoku_static_struct());
    }

    #[test]
    fn parse_input_into_sudoku_ok_zeroes() {
        let mut input = build_sudoku_static_with_zeroes()
            .into_iter()
            .map(|item| item as SNumber)
            .collect::<Vec<SNumber>>();
        let parsed = Sudoku::parse_input_into_sudoku(&mut input);

        assert_eq!(parsed.unwrap(), build_sudoku_static_struct_with_zeroes());
    }

    #[rustfmt::skip]
    fn build_sudoku_static() -> Vec<InputNumber> {
        vec![
            8, 2, 7,  1, 5, 4,  3, 9, 6,
            9, 6, 5,  3, 2, 7,  1, 4, 8,
            3, 4, 1,  6, 8, 9,  7, 5, 2,

            5, 9, 3,  4, 6, 8,  2, 7, 1,
            4, 7, 2,  5, 1, 3,  6, 8, 9,
            6, 1, 8,  9, 7, 2,  4, 3, 5,

            7, 8, 6,  2, 3, 5,  9, 1, 4,
            1, 5, 4,  7, 9, 6,  8, 2, 3,
            2, 3, 9,  8, 4, 1,  5, 6, 7
        ]
    }

    fn build_sudoku_static_struct() -> Sudoku {
        Sudoku {
            q1: Quadrant {
                c1: 8,
                c2: 2,
                c3: 7,
                c4: 9,
                c5: 6,
                c6: 5,
                c7: 3,
                c8: 4,
                c9: 1,
            },
            q2: Quadrant {
                c1: 1,
                c2: 5,
                c3: 4,
                c4: 3,
                c5: 2,
                c6: 7,
                c7: 6,
                c8: 8,
                c9: 9,
            },
            q3: Quadrant {
                c1: 3,
                c2: 9,
                c3: 6,
                c4: 1,
                c5: 4,
                c6: 8,
                c7: 7,
                c8: 5,
                c9: 2,
            },
            q4: Quadrant {
                c1: 5,
                c2: 9,
                c3: 3,
                c4: 4,
                c5: 7,
                c6: 2,
                c7: 6,
                c8: 1,
                c9: 8,
            },
            q5: Quadrant {
                c1: 4,
                c2: 6,
                c3: 8,
                c4: 5,
                c5: 1,
                c6: 3,
                c7: 9,
                c8: 7,
                c9: 2,
            },
            q6: Quadrant {
                c1: 2,
                c2: 7,
                c3: 1,
                c4: 6,
                c5: 8,
                c6: 9,
                c7: 4,
                c8: 3,
                c9: 5,
            },
            q7: Quadrant {
                c1: 7,
                c2: 8,
                c3: 6,
                c4: 1,
                c5: 5,
                c6: 4,
                c7: 2,
                c8: 3,
                c9: 9,
            },
            q8: Quadrant {
                c1: 2,
                c2: 3,
                c3: 5,
                c4: 7,
                c5: 9,
                c6: 6,
                c7: 8,
                c8: 4,
                c9: 1,
            },
            q9: Quadrant {
                c1: 9,
                c2: 1,
                c3: 4,
                c4: 8,
                c5: 2,
                c6: 3,
                c7: 5,
                c8: 6,
                c9: 7,
            },
        }
    }

    #[rustfmt::skip]
    fn build_sudoku_static_with_zeroes() -> Vec<InputNumber> {
        vec![
            5, 3, 0, 0, 7, 0, 0, 0, 0,
            6, 0, 0, 1, 9, 5, 0, 0, 0,
            0, 9, 8, 0, 0, 0, 0, 6, 0,
            8, 0, 0, 0, 6, 0, 0, 0, 3,
            4, 0, 0, 8, 0, 3, 0, 0, 1,
            7, 0, 0, 0, 2, 0, 0, 0, 6,
            0, 6, 0, 0, 0, 0, 2, 8, 0,
            0, 0, 0, 4, 1, 9, 0, 0, 5,
            0, 0, 0, 0, 8, 0, 0, 7, 9,
        ]
    }

    fn build_sudoku_static_struct_with_zeroes() -> Sudoku {
        Sudoku {
            q1: Quadrant {
                c1: 5,
                c2: 3,
                c3: 0,
                c4: 6,
                c5: 0,
                c6: 0,
                c7: 0,
                c8: 9,
                c9: 8,
            },
            q2: Quadrant {
                c1: 0,
                c2: 7,
                c3: 0,
                c4: 1,
                c5: 9,
                c6: 5,
                c7: 0,
                c8: 0,
                c9: 0,
            },
            q3: Quadrant {
                c1: 0,
                c2: 0,
                c3: 0,
                c4: 0,
                c5: 0,
                c6: 0,
                c7: 0,
                c8: 6,
                c9: 0,
            },
            q4: Quadrant {
                c1: 8,
                c2: 0,
                c3: 0,
                c4: 4,
                c5: 0,
                c6: 0,
                c7: 7,
                c8: 0,
                c9: 0,
            },
            q5: Quadrant {
                c1: 0,
                c2: 6,
                c3: 0,
                c4: 8,
                c5: 0,
                c6: 3,
                c7: 0,
                c8: 2,
                c9: 0,
            },
            q6: Quadrant {
                c1: 0,
                c2: 0,
                c3: 3,
                c4: 0,
                c5: 0,
                c6: 1,
                c7: 0,
                c8: 0,
                c9: 6,
            },
            q7: Quadrant {
                c1: 0,
                c2: 6,
                c3: 0,
                c4: 0,
                c5: 0,
                c6: 0,
                c7: 0,
                c8: 0,
                c9: 0,
            },
            q8: Quadrant {
                c1: 0,
                c2: 0,
                c3: 0,
                c4: 4,
                c5: 1,
                c6: 9,
                c7: 0,
                c8: 8,
                c9: 0,
            },
            q9: Quadrant {
                c1: 2,
                c2: 8,
                c3: 0,
                c4: 0,
                c5: 0,
                c6: 5,
                c7: 0,
                c8: 7,
                c9: 9,
            },
        }
    }
}
