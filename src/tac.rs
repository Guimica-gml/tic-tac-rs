use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    None,
    X,
    O,
}

impl Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::None => write!(f, " "),
            State::X => write!(f, "X"),
            State::O => write!(f, "O"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TicTacToe {
    width: usize,
    height: usize,
    cells: Vec<Vec<State>>,
    turn: State,
}

impl TicTacToe {
    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }
    pub fn get_cell(&self, x: usize, y: usize) -> &State { &self.cells[y][x] }
    pub fn turn(&self) -> State { self.turn }

    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width, height,
            cells: vec![vec![State::None; width]; height],
            turn: State::X,
        }
    }

    pub fn next(&mut self, x: usize, y: usize) {
        assert!(x < self.width && y < self.height, "Out of bounds");
        if let State::None = self.cells[y][x] {
            self.cells[y][x] = self.turn;
            self.turn = if self.turn == State::X { State::O } else { State::X };
        }
    }

    pub fn is_cell_empty(&self, x: usize, y: usize) -> bool {
        assert!(x < self.width && y < self.height, "Out of bounds");
        self.cells[y][x] == State::None
    }

    pub fn check_winner(&self) -> State {
        for y in 0..self.height {
            for x in 0..self.width {
                let state = self.cells[y][x];
                if let State::None = state {
                    continue;
                }

                if x + 2 < self.width {
                    if (1..3).all(|i| self.cells[y][x + i] == state) {
                        return state;
                    }
                }

                if y + 2 < self.height {
                    if (1..3).all(|i| self.cells[y + i][x] == state) {
                        return state;
                    }
                }

                if x + 2 < self.width && y + 2 < self.height {
                    if (1..3).all(|i| self.cells[y + i][x + i] == state) {
                        return state;
                    }
                }

                if x as i32 - 2 >= 0 && y + 2 < self.height {
                    if (1..3).all(|i| self.cells[y + i][x - i] == state) {
                        return state;
                    }
                }
            }
        }

        State::None
    }

    pub fn is_full(&self) -> bool {
        for y in 0..self.height {
            for x in 0..self.width {
                if let State::None = self.cells[y][x] {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn reset(&mut self) {
        self.turn = State::X;
        for y in 0..self.height {
            for x in 0..self.width {
                self.cells[y][x] = State::None;
            }
        }
    }
}
