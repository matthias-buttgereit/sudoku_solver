use std::{collections::HashMap, time::Instant};

#[derive(Debug, PartialEq, Eq)]
pub struct Sudoku {
    board: Vec<Vec<u8>>,
}

impl Sudoku {
    fn check_if_n_valid(&self, x_position: usize, y_position: usize, number: u8) -> bool {
        for x in 0..9 {
            if self.board[x][y_position] == number {
                return false;
            }
        }

        for y in 0..9 {
            if self.board[x_position][y] == number {
                return false;
            }
        }

        for x in x_position / 3 * 3..x_position / 3 * 3 + 3 {
            for y in y_position / 3 * 3..y_position / 3 * 3 + 3 {
                if self.board[x][y] == number {
                    return false;
                }
            }
        }

        true
    }

    fn find_empty_positions(&self) -> Vec<(usize, usize)> {
        let mut list = Vec::new();
        for x in 0..9 {
            for y in 0..9 {
                if self.board[x][y] == 0 {
                    list.push((x, y));
                }
            }
        }
        list
    }

    fn find_valid_numbers_for(
        &self,
        list: &Vec<(usize, usize)>,
    ) -> HashMap<(usize, usize), Vec<u8>> {
        let mut hashmap = HashMap::new();
        for (x, y) in list {
            for number in 1..=9 {
                if self.check_if_n_valid(*x, *y, number) {
                    let list = hashmap.entry((*x, *y)).or_insert(Vec::new());
                    list.push(number);
                }
            }
        }

        hashmap
    }

    pub fn new() -> Sudoku {
        println!("Created new Sudoku game.\n");
        Sudoku {
            board: vec![vec![0; 9]; 9],
        }
    }

    pub fn from_string(string: &str) -> Self {
        let mut sudoku = Self::new();

        for (x, line) in string.lines().enumerate() {
            for (y, number) in line.chars().enumerate() {
                let number = number.to_digit(10).expect("Invalid number in String.") as u8;

                sudoku.set_value(x, y, number);
            }
        }

        sudoku
    }

    pub fn print(&self) {
        for (x, col) in self.board.iter().enumerate() {
            for (y, value) in col.iter().enumerate() {
                if *value == 0 {
                    print!("  ");
                } else {
                    print!("{} ", value);
                }

                if y == 2 || y == 5 {
                    print!("| ")
                }
            }
            if x == 2 || x == 5 {
                print!("\n- - - + - - - + - - -")
            }
            println!();
        }
        println!();
    }

    pub fn set_board(&mut self, board: Vec<Vec<u8>>) -> Result<(), String> {
        if board.len() != 9 {
            return Err(String::from("Dimensions must be 9x9."));
        }
        for line in &board {
            if line.len() != 9 {
                return Err(String::from("Dimensions must be 9x9."));
            }
        }
        self.board = board;
        Ok(())
    }

    pub fn set_value(&mut self, x: usize, y: usize, number: u8) {
        if x >= 9 || y >= 9 || number > 9 {
            println!("Numbers must be between 1 and 9.")
        } else {
            self.board[x][y] = number;
        }
    }

    pub fn solve(&mut self) -> bool {
        println!("Solving Sudoku...");
        let start = Instant::now();
        let empty_positions = self.find_empty_positions();
        let valid_numbers = self.find_valid_numbers_for(&empty_positions);

        match self.solve_recursive(&empty_positions, &valid_numbers) {
            true => {
                let time_elapsed = Instant::elapsed(&start);
                println!("Solved in {} ms.\n", time_elapsed.as_millis());
                true
            }
            false => {
                let time_elapsed = Instant::elapsed(&start);
                println!("No solution found after {} ms.\n", time_elapsed.as_millis());
                false
            }
        }
    }

    fn solve_recursive(
        &mut self,
        position: &[(usize, usize)],
        numbers: &HashMap<(usize, usize), Vec<u8>>,
    ) -> bool {
        if position.is_empty() {
            return true;
        }

        let (x, y) = position[0];
        for number in numbers.get(&position[0]).unwrap() {
            if self.check_if_n_valid(x, y, *number) {
                self.board[x][y] = *number;
                if self.solve_recursive(&position[1..], numbers) {
                    return true;
                }
            }
        }
        self.board[x][y] = 0;

        false
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn put_number_in_first_position() {
        let mut board = vec![vec![0; 9]; 9];
        board[0][0] = 9;
        let mut game = Sudoku::new();

        let _result = game.set_value(0, 0, 9);
        assert_eq!(game.board, board);
    }

    #[test]
    fn put_number_in_last_position() {
        let mut board = vec![vec![0; 9]; 9];
        board[8][8] = 1;
        let mut game = Sudoku::new();

        let _result = game.set_value(8, 8, 1);
        assert_eq!(game.board, board);
    }

    #[test]
    fn solve_sudoku() {
        let mut sudoku = Sudoku::new();

        sudoku
            .set_board(vec![
                vec![0, 4, 0, 5, 0, 0, 8, 3, 7],
                vec![0, 5, 7, 9, 8, 2, 1, 6, 4],
                vec![6, 0, 0, 0, 7, 0, 2, 0, 5],
                vec![2, 6, 0, 7, 1, 4, 0, 0, 3],
                vec![0, 9, 0, 0, 0, 0, 0, 0, 1],
                vec![0, 0, 0, 6, 3, 0, 4, 5, 2],
                vec![5, 0, 9, 0, 4, 0, 3, 0, 8],
                vec![8, 2, 4, 3, 0, 0, 0, 0, 6],
                vec![1, 3, 0, 0, 0, 5, 0, 4, 9],
            ])
            .unwrap();

        sudoku.solve();

        assert_eq!(
            sudoku.board,
            vec![
                vec![9, 4, 2, 5, 6, 1, 8, 3, 7],
                vec![3, 5, 7, 9, 8, 2, 1, 6, 4],
                vec![6, 1, 8, 4, 7, 3, 2, 9, 5],
                vec![2, 6, 5, 7, 1, 4, 9, 8, 3],
                vec![4, 9, 3, 2, 5, 8, 6, 7, 1],
                vec![7, 8, 1, 6, 3, 9, 4, 5, 2],
                vec![5, 7, 9, 1, 4, 6, 3, 2, 8],
                vec![8, 2, 4, 3, 9, 7, 5, 1, 6],
                vec![1, 3, 6, 8, 2, 5, 7, 4, 9],
            ]
        )
    }

    #[test]
    fn actual_sudoku() {
        let mut sudoku = Sudoku::new();

        sudoku
            .set_board(vec![
                vec![2, 0, 3, 0, 5, 0, 1, 0, 0],
                vec![0, 0, 0, 4, 0, 0, 0, 0, 7],
                vec![0, 9, 0, 0, 0, 0, 0, 0, 0],
                vec![0, 6, 0, 0, 8, 0, 0, 0, 0],
                vec![9, 0, 8, 0, 0, 7, 0, 0, 1],
                vec![0, 5, 0, 0, 0, 0, 9, 0, 0],
                vec![8, 0, 1, 0, 3, 0, 2, 0, 0],
                vec![0, 0, 6, 0, 0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 0, 5, 0, 3, 0],
            ])
            .unwrap();

        sudoku.print();

        sudoku.solve();

        sudoku.print();
    }

    #[test]
    fn create_sudoku_from_string() {
        let str_sudoku = Sudoku::from_string("1");
        let mut sudoku = Sudoku::new();
        sudoku.set_value(0, 0, 1);

        assert_eq!(str_sudoku, sudoku);
    }

    #[test]
    #[should_panic]
    fn create_sudoku_from_string_with_too_high_value() {
        let str_sudoku = Sudoku::from_string("19");
        
    }
}
