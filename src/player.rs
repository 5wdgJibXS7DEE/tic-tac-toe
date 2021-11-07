use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};

pub enum PlayerKind {
    HUMAN,
    AI,
}

pub struct Player {
    pub token: char,
    pub cases: i32,
    pub kind: PlayerKind,
    turns: Vec<i32>,
}

impl Player {
    pub fn human(token: char) -> Player {
        Player {
            token: token,
            cases: 0,
            kind: PlayerKind::HUMAN,
            turns: vec![],
        }
    }

    pub fn ai(token: char) -> Player {
        Player {
            token: token,
            cases: 0,
            kind: PlayerKind::AI,
            turns: generate_random_turns(),
        }
    }

    pub fn play(&mut self, board: i32) {
        let mut case_as_flag;
        loop {
            let case = match self.kind {
                PlayerKind::HUMAN => {
                    print!("Enter a case for {}: ", self.token);
                    io::stdout().flush().unwrap();
                    read_i32_from_stdin_or_throw()
                }
                PlayerKind::AI => self.turns.pop().unwrap(),
            };

            case_as_flag = 1 << case;
            if case_as_flag & board == 0 {
                if matches!(self.kind, PlayerKind::AI) {
                    println!("Computer plays case {}.", case);
                }

                break;
            }

            if matches!(self.kind, PlayerKind::HUMAN) {
                println!("This case is already taken.");
            }
        }

        self.cases |= case_as_flag;
    }
}

fn generate_random_turns() -> Vec<i32> {
    let mut ai_turns: Vec<i32> = (0..9).collect();
    ai_turns.shuffle(&mut thread_rng());
    ai_turns
}

fn read_i32_from_stdin_or_throw() -> i32 {
    let mut buffer = String::new();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("The program failed to read your input");

    trim_newline(&mut buffer);

    let parsed: i32 = buffer.parse().unwrap();
    parsed
}

fn trim_newline(s: &mut String) {
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
    }
}
