use std::collections::HashMap;
use super::tac::*;

pub struct TicTacToeAi {
    cache: HashMap<TicTacToe, i32>,
}

impl TicTacToeAi {
    pub fn new() -> Self {
        Self { cache: HashMap::new() }
    }

    pub fn minimax(&mut self, tic_tac_toe: TicTacToe) -> i32 {
        if self.cache.contains_key(&tic_tac_toe) {
            return self.cache[&tic_tac_toe];
        }

        match tic_tac_toe.check_winner() {
            State::X => return 1,
            State::O => return -1,
            _ if tic_tac_toe.is_full() => return 0,
            _ => {}
        }

        let mut best_score = if tic_tac_toe.turn() == State::X { i32::MIN } else { i32::MAX };

        for y in 0..tic_tac_toe.height() {
            for x in 0..tic_tac_toe.width() {
                if !tic_tac_toe.is_cell_empty(x, y) {
                    continue;
                }

                let mut clone = tic_tac_toe.clone();
                clone.next(x, y);
                let score = self.minimax(clone);
                best_score = if tic_tac_toe.turn() == State::X {
                    i32::max(best_score, score)
                }
                else {
                    i32::min(best_score, score)
                }
            }
        }

        self.cache.insert(tic_tac_toe, best_score);
        best_score
    }

    pub fn next_move(&mut self, tic_tac_toe: &TicTacToe) -> Option<(usize, usize)> {
        match tic_tac_toe.check_winner() {
            State::X | State::O => return None,
            _ if tic_tac_toe.is_full() => return None,
            _ => {}
        }

        let mut best_score = if tic_tac_toe.turn() == State::X { i32::MIN } else { i32::MAX };
        let mut best_move: (usize, usize) = (0, 0);

        for y in 0..tic_tac_toe.height() {
            for x in 0..tic_tac_toe.width() {
                if !tic_tac_toe.is_cell_empty(x, y) {
                    continue;
                }

                let mut clone = tic_tac_toe.clone();
                clone.next(x, y);
                let score = self.minimax(clone);

                if tic_tac_toe.turn() == State::X {
                    if score > best_score {
                        best_score = score;
                        best_move = (x, y);
                    }
                }
                else {
                    if score < best_score {
                        best_score = score;
                        best_move = (x, y);
                    }
                };
            }
        }

        Some(best_move)
    }
}
