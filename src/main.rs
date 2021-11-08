mod player;
use player::*;
mod configuration;
use clap::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let (human, ai) = read_cl();
    let mut players = create(human, ai);
    welcome(&players);

    let winner = run_game(&mut players);
    congratulate(winner);
    print_board(&players);
}

fn read_cl() -> (u8, u8) {
    let matches = App::new("Tic-tac-toe with Rust")
        .version("0.2")
        .about("Play with your friends and computers")
        .arg(
            Arg::with_name("humans")
                .long("humans")
                .value_name("n")
                .help("Sets the number of human players.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("computers")
                .long("computers")
                .value_name("n")
                .help("Sets the number of computer players.")
                .takes_value(true),
        )
        .get_matches();

    let mut human: u8 = match matches.value_of("humans") {
        Some(n) => n.parse().expect("invalid number for argument: humans"),
        None => 0,
    };

    let mut ai: u8 = match matches.value_of("computers") {
        Some(n) => n.parse().expect("invalid number for argument: computers"),
        None => 0,
    };

    if human + ai > 26 {
        panic!("total number of players is limited to 26");
    }

    // sets default number if no # players is provided
    if human + ai == 0 {
        human = 1;
        ai = 1;
    }

    (human, ai)
}

fn create(human: u8, ai: u8) -> Vec<Player> {
    let mut players: Vec<Player> = vec![];
    let mut token: char = 'A';

    for _ in 0..human {
        players.push(Player::human(token));
        token = ((token as u8) + 1) as char;
    }

    for _ in 0..ai {
        players.push(Player::ai(token));
        token = ((token as u8) + 1) as char;
    }

    players.shuffle(&mut thread_rng());
    players
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

    for row in 0..configuration::BOARD_ROWS {
        print!("|");

        // print the left board with cases' number
        for col in 0..configuration::BOARD_ROW_SIZE {
            print!(" {} |", row * configuration::BOARD_ROWS + col);
        }

        print!("  =>  |");

        // print the right board with tokens occupying cases
        for col in 0..configuration::BOARD_ROW_SIZE {
            let case = row * configuration::BOARD_ROWS + col;
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
                None => print!("   |"),
            }
        }

        println!();
    }
}
