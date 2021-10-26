use integer_sqrt::IntegerSquareRoot;
use std::io::{self, Write};

#[derive(Clone)]
enum Case {
    Empty,
    Taken(char),
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    NextTurn,
    InvalidTurn,
    Draw,
    Winner(char),
}

pub struct Board {
    length: usize,
    cases: Vec<Case>,
    state: State,
    n_filled: usize,
}

impl Board {
    pub fn new(size: u8) -> Board {
        let length: usize = (size * size) as usize; // todo
        let cases = vec![Case::Empty; length];

        Board {
            length: length,
            state: State::NextTurn,
            cases: cases,
            n_filled: 0,
        }
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn fill(&mut self, at: u16, symbol: char) {
        if !matches!(self.cases[at as usize], Case::Empty) {
            self.state = State::InvalidTurn;
        } else {
            self.cases[at as usize] = Case::Taken(symbol);
            self.n_filled += 1;

            if self.n_filled == self.length {
                self.state = State::Draw;
                return;
            }

            if self.is_winner(symbol) {
                self.state = State::Winner(symbol);
                return;
            }

            self.state = State::NextTurn;
        }
    }

    fn is_winner(&self, symbol: char) -> bool {
        if self.is_winner_horizontal(symbol) {
            return true; // todo
        }

        if self.is_winner_vertical(symbol) {
            return true; // todo
        }

        if self.is_winner_diagonal(symbol) {
            return true; // todo
        }

        false
    }

    fn is_winner_horizontal(&self, symbol: char) -> bool {
        false
    }

    fn is_winner_vertical(&self, symbol: char) -> bool {
        false
    }

    fn is_winner_diagonal(&self, symbol: char) -> bool {
        false
    }

    pub fn print(&self) {
        let sq_length = self.length.integer_sqrt();

        for at in 0..self.length {
            if (at != 0) && (at % sq_length == 0) {
                println!();
            }

            match self.cases[at] {
                Case::Taken(symbol) => print!(" {}|", symbol),
                _ => print!("{:02x}|", at),
            }
        }
        println!();

        io::stdout().flush().unwrap();
    }
}
