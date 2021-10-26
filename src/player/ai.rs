use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Ai {
    symbol: char,
    moves: Vec<u16>,
}

impl Ai {
    pub fn new(board_size: u8, symbol: char) -> Ai {
        let sq_size: usize = (board_size * board_size) as usize; // todo
        let mut moves: Vec<u16> = (0..sq_size as u16).collect();
        moves.shuffle(&mut thread_rng());

        Ai {
            symbol: symbol,
            moves: moves,
        }
    }
}

impl super::Play for Ai {
    fn get_symbol(&self) -> char {
        self.symbol
    }

    fn play(&mut self) -> u16 {
        let at = self.moves.pop().unwrap();
        println!("Computer turn {}: {:02x}", self.symbol, at);
        at
    }
}