use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};

use crate::configuration;

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

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
                    read_case_from_stdin(&format!("Enter a case for {}: ", self.token))
                }
                PlayerKind::AI => self.turns.pop().unwrap(),
            };

            // break the loop when the chosen case is valid (not taken)
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
    let mut ai_turns: Vec<i32> =
        (configuration::BOARD_MIN_CASE..=configuration::BOARD_MAX_CASE).collect();
    ai_turns.shuffle(&mut thread_rng());
    ai_turns
}

fn read_case_from_stdin(msg: &str) -> i32 {
    loop {
        print!("{}", msg);
        io::stdout()
            .flush()
            .expect("the program failed to flush output");

        match read_i32_from_stdin() {
            Ok(case) => return case,
            Err(err) => println!("{}", err),
        }
    }
}

fn read_i32_from_stdin() -> GenericResult<i32> {
    let mut buffer = String::new();

    std::io::stdin()
        .read_line(&mut buffer)
        .expect("the program failed to read your input");

    trim_newline(&mut buffer);
    let buffer = buffer.trim();

    let parsed: i32 = buffer.parse()?;

    if parsed < configuration::BOARD_MIN_CASE || parsed > configuration::BOARD_MAX_CASE {
        return Err(GenericError::from("number does not fit the grid"));
    }

    Ok(parsed)
}

fn trim_newline(s: &mut String) {
    while s.ends_with('\n') || s.ends_with('\r') {
        s.pop();
    }
}
