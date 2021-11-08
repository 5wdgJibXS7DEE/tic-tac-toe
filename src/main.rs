mod player;
use player::*;

mod configuration;

fn main() {
    let mut players = [
        Player::human('A'),
        Player::ai('B'),
        Player::ai('C'),
        Player::human('D'),
    ];
    welcome(&players);

    let winner = run_game(&mut players);
    congratulate(winner);
}

fn welcome(players: &[Player]) {
    let get_player_kind = |x: &Player| match x.kind {
        PlayerKind::HUMAN => "You",
        PlayerKind::AI => "Computer",
    };

    println!("Welcome to Rust tic-tac-toe, have fun!");
    println!(
        "{} plays first with {}.",
        get_player_kind(&players[0]),
        players[0].token
    );

    for i in 1..players.len() {
        println!(
            "{} plays then with {}.",
            get_player_kind(&players[i]),
            players[i].token
        );
    }
}

fn run_game(players: &mut [Player]) -> Option<&Player> {
    let mut turn = 0;

    loop {
        if matches!(players[turn].kind, PlayerKind::HUMAN) {
            print_board(players);
        }

        let board = compute_board(players);
        players[turn].play(board);
        let board = compute_board(players);

        if is_winner(players[turn].cases) {
            return Some(&players[turn]);
        }

        if board == configuration::BOARD_FULL {
            return None;
        }

        turn = (turn + 1) % players.len();
    }
}

fn congratulate(winner: Option<&Player>) {
    match winner {
        Some(player) => {
            if matches!(player.kind, PlayerKind::HUMAN) {
                println!("{} won, congratulations!", player.token);
            } else {
                println!("Computer {} won, try again :)", player.token);
            }
        }
        None => println!("It's a draw, try again :)"),
    }
}

fn is_winner(cases: i32) -> bool {
    for s in configuration::SOLUTIONS {
        if cases & s == s {
            return true;
        }
    }
    false
}

fn compute_board(players: &[Player]) -> i32 {
    players.iter().fold(0, |acc, p| acc | p.cases)
}

fn print_board(players: &[Player]) {
    println!("");
    println!("Board:");

    for case in configuration::BOARD_MIN_CASE..=configuration::BOARD_MAX_CASE {
        let case_as_flag = 1 << case;

        let mut occupied: Option<char> = None;
        for p in players {
            if case_as_flag & p.cases != 0 {
                occupied = Some(p.token);
                break;
            }
        }

        match occupied {
            Some(token) => print!(" {} |", token),
            None => print!(" {} |", case),
        }

        if (case + 1) % configuration::BOARD_ROW_SIZE == 0 {
            println!();
        }
    }
}
