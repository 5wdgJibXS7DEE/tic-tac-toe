use integer_sqrt::IntegerSquareRoot;
use std::io::{self, Write};

#[derive(Clone, PartialEq, Eq)]
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

            if self.is_winner(symbol) {
                self.state = State::Winner(symbol);
                return;
            }

            if self.n_filled == self.length {
                self.state = State::Draw;
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
        let sq_length = self.length.integer_sqrt();
        let mut count = 0;

        for at in 0..self.length {
            // new line reset the counter
            if at % sq_length == 0 {
                count = 0;
            }

            // increment counter if same symbol, reset it otherwise
            if self.cases[at] == Case::Taken(symbol) {
                count += 1;
            }
            else {
                count = 0;
            }

            // todo number of cases to win (here sq_length) could be easily parametrized
            if count == sq_length {
                return true;
            }
        }

        false
    }

    fn is_winner_vertical(&self, symbol: char) -> bool {
        let sq_length = self.length.integer_sqrt();
        let mut count = 0;

        // todo need rework if we want to parametried the number of cases to win
        for col in 0..sq_length {
            count = 0;

            for row in 0..sq_length {
                let at = row * sq_length + col;

                if self.cases[at] == Case::Taken(symbol) {
                    count += 1;
                }
                else {
                    count = 0;
                }
            }

            if count == sq_length {
                return true;
            }
        }

        false
    }

    fn is_winner_diagonal(&self, symbol: char) -> bool {
        let sq_length = self.length.integer_sqrt();
        let mut count = 0;

        // todo need rework if we want to parametried the number of cases to win
        for i in 0..sq_length {
            let at = i * sq_length + i;

            if self.cases[at] == Case::Taken(symbol) {
                count += 1;
            }
        }

        if count == sq_length {
            return true;
        }

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
