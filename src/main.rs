mod board;
mod player;

use board::*;
use player::{ai::Ai, human::Human, *};

fn main() {
    let board_size: u8 = 4;

    let mut board = Board::new(board_size);
    let mut ai = Ai::new(board_size, 'y'); // spade suit symbol
    let mut human = Human::new('z'); // heart suit symbol

    const N_PLAYERS: usize = 2;
    let players: [&mut dyn Play; N_PLAYERS] = [&mut ai, &mut human];
    let mut cur_turn = 0;
    let mut cur_player: &mut dyn Play = players[0]; // todo init is not really necesary

    loop {
        match board.get_state() {
            State::NextTurn => {
                // todo why trying to copy if accessing a field of the struct?
                board.print();

                cur_turn = (cur_turn + 1) % N_PLAYERS;
                cur_player = players[cur_turn];

                let at = cur_player.play();
                let symbol = cur_player.get_symbol();

                board.fill(at, symbol);
            }
            State::InvalidTurn => {
                // todo gather in a method
                let at = &cur_player.play();
                let symbol = &cur_player.get_symbol();
                board.fill(*at, *symbol);
            }
            _ => break,
        }
    }

    board.print();
    println!("The game finished with state {:?}", board.get_state());
}
