use rand::seq::SliceRandom;

const SIZE: usize = 9;
const BOX: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn remove_count(self) -> usize {
        match self {
            Difficulty::Easy => 35,
            Difficulty::Medium => 45,
            Difficulty::Hard => 55,
        }
    }

    pub fn next(self) -> Self {
        match self {
            Difficulty::Easy => Difficulty::Medium,
            Difficulty::Medium => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Easy,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Difficulty::Easy => "Facil",
            Difficulty::Medium => "Medio",
            Difficulty::Hard => "Dificil",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sudoku {
    pub solution: [[u8; SIZE]; SIZE],
    pub board: [[u8; SIZE]; SIZE],
    pub initial: [[bool; SIZE]; SIZE],
}

impl Sudoku {
    pub fn new(difficulty: Difficulty) -> Self {
        let mut rng = rand::rng();
        let solution = generate(&mut rng);
        let mut board = solution;
        let mut initial = [[false; SIZE]; SIZE];

        let mut cells: Vec<(usize, usize)> = (0..SIZE)
            .flat_map(|r| (0..SIZE).map(move |c| (r, c)))
            .collect();
        cells.shuffle(&mut rng);

        let remove_count = difficulty.remove_count();
        for &(r, c) in cells.iter().take(remove_count) {
            board[r][c] = 0;
        }

        for r in 0..SIZE {
            for c in 0..SIZE {
                initial[r][c] = board[r][c] != 0;
            }
        }

        Sudoku {
            solution,
            board,
            initial,
        }
    }

    pub fn is_complete(&self) -> bool {
        for r in 0..SIZE {
            for c in 0..SIZE {
                if self.board[r][c] != self.solution[r][c] {
                    return false;
                }
            }
        }
        true
    }

    pub fn row_has_duplicate(&self, row: usize) -> bool {
        let mut seen = [false; SIZE + 1];
        for c in 0..SIZE {
            let num = self.board[row][c] as usize;
            if num == 0 {
                continue;
            }
            if seen[num] {
                return true;
            }
            seen[num] = true;
        }
        false
    }

    pub fn col_has_duplicate(&self, col: usize) -> bool {
        let mut seen = [false; SIZE + 1];
        for r in 0..SIZE {
            let num = self.board[r][col] as usize;
            if num == 0 {
                continue;
            }
            if seen[num] {
                return true;
            }
            seen[num] = true;
        }
        false
    }

    pub fn block_has_duplicate(&self, row: usize, col: usize) -> bool {
        let br = (row / BOX) * BOX;
        let bc = (col / BOX) * BOX;
        let mut seen = [false; SIZE + 1];
        for r in br..br + BOX {
            for c in bc..bc + BOX {
                let num = self.board[r][c] as usize;
                if num == 0 {
                    continue;
                }
                if seen[num] {
                    return true;
                }
                seen[num] = true;
            }
        }
        false
    }

    pub fn cell_has_duplicate(&self, row: usize, col: usize) -> bool {
        let num = self.board[row][col];
        if num == 0 {
            return false;
        }
        // Check row
        for c in 0..SIZE {
            if c != col && self.board[row][c] == num {
                return true;
            }
        }
        // Check col
        for r in 0..SIZE {
            if r != row && self.board[r][col] == num {
                return true;
            }
        }
        // Check block
        let br = (row / BOX) * BOX;
        let bc = (col / BOX) * BOX;
        for r in br..br + BOX {
            for c in bc..bc + BOX {
                if (r, c) != (row, col) && self.board[r][c] == num {
                    return true;
                }
            }
        }
        false
    }

    pub fn set_cell(&mut self, row: usize, col: usize, num: u8) {
        self.board[row][col] = num;
    }

    pub fn clear_cell(&mut self, row: usize, col: usize) {
        if !self.initial[row][col] {
            self.board[row][col] = 0;
        }
    }

    pub fn hint(&mut self, row: usize, col: usize) {
        if !self.initial[row][col] {
            self.board[row][col] = self.solution[row][col];
        }
    }
}

fn generate(rng: &mut impl rand::Rng) -> [[u8; SIZE]; SIZE] {
    let mut board = [[0u8; SIZE]; SIZE];
    fill_board(&mut board, rng);
    board
}

fn fill_board(board: &mut [[u8; SIZE]; SIZE], rng: &mut impl rand::Rng) -> bool {
    let Some((r, c)) = find_empty(board) else {
        return true;
    };

    let mut nums: Vec<u8> = (1..=9).collect();
    nums.shuffle(rng);

    for num in nums {
        if is_safe(board, r, c, num) {
            board[r][c] = num;
            if fill_board(board, rng) {
                return true;
            }
            board[r][c] = 0;
        }
    }
    false
}

fn find_empty(board: &[[u8; SIZE]; SIZE]) -> Option<(usize, usize)> {
    for (r, row) in board.iter().enumerate().take(SIZE) {
        for (c, &val) in row.iter().enumerate().take(SIZE) {
            if val == 0 {
                return Some((r, c));
            }
        }
    }
    None
}

fn is_safe(board: &[[u8; SIZE]; SIZE], row: usize, col: usize, num: u8) -> bool {
    for i in 0..SIZE {
        if board[row][i] == num {
            return false;
        }
        if board[i][col] == num {
            return false;
        }
    }
    let br: usize = (row / BOX) * BOX;
    let bc = (col / BOX) * BOX;
    for r in br..br + BOX {
        for c in bc..bc + BOX {
            if board[r][c] == num {
                return false;
            }
        }
    }
    true
}
