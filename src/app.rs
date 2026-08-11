use crate::game::{Difficulty, Sudoku};

pub struct App {
    pub game: Sudoku,
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub difficulty: Difficulty,
    pub exit: bool,
    pub won: bool,
}

impl App {
    pub fn new(difficulty: Difficulty) -> Self {
        App {
            game: Sudoku::new(difficulty),
            cursor_row: 0,
            cursor_col: 0,
            difficulty,
            exit: false,
            won: false,
        }
    }

    pub fn new_game(&mut self) {
        self.game = Sudoku::new(self.difficulty);
        self.cursor_row = 0;
        self.cursor_col = 0;
        self.won = false;
    }

    pub fn change_difficulty(&mut self) {
        self.difficulty = self.difficulty.next();
        self.new_game();
    }

    pub fn move_up(&mut self) {
        self.cursor_row = self.cursor_row.saturating_sub(1);
    }

    pub fn move_down(&mut self) {
        if self.cursor_row < 8 {
            self.cursor_row += 1;
        }
    }

    pub fn move_left(&mut self) {
        self.cursor_col = self.cursor_col.saturating_sub(1);
    }

    pub fn move_right(&mut self) {
        if self.cursor_col < 8 {
            self.cursor_col += 1;
        }
    }

    pub fn input_number(&mut self, num: u8) {
        if self.game.initial[self.cursor_row][self.cursor_col] {
            return;
        }
        self.game.set_cell(self.cursor_row, self.cursor_col, num);
        if self.game.is_complete() {
            self.won = true;
        }
    }

    pub fn clear_cell(&mut self) {
        self.game.clear_cell(self.cursor_row, self.cursor_col);
    }

    pub fn hint(&mut self) {
        self.game.hint(self.cursor_row, self.cursor_col);
        if self.game.is_complete() {
            self.won = true;
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new(Difficulty::Easy)
    }
}
