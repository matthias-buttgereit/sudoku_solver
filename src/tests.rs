#[cfg(test)]
mod tests {
    use sudoku_solver::Sudoku;

    #[test]
    fn put_number_in_first_position() {
        let game = Sudoku::new();

        game.set_value(0, 0, 9);
    }

    #[test]
    fn put_number_in_first_position() {
        let game = Sudoku::new();

        game.set_value(9, 9, 1);
    }

}