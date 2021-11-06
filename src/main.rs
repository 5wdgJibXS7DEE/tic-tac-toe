use std::io::{self, Write};
use rand::seq::SliceRandom;
use rand::thread_rng;

const PLAYER_TOKEN: char = 'X';
const AI_TOKEN: char = 'Y';

static SOLUTIONS: [i32; 9] = [0b111_111_111,     // full
    0b000_000_111, 0b000_111_000, 0b111_000_000, // rows
    0b001_001_001, 0b010_010_010, 0b100_100_100, // columns
    0b100_010_001, 0b001_010_100];               // diagonals

enum Winner {
    DRAW,
    PLAYER,
    AI,
}

fn main() {
    print_welcome();
    let (winner, player_cases, ai_cases) = run();
    print_winner(winner, player_cases, ai_cases);
}

fn print_welcome() {
    println!("Welcome to Rust tic-tac-toe, have fun!");
    println!("You play with: {}.", PLAYER_TOKEN);
    println!("IA plays with: {}.", AI_TOKEN);
}

fn print_winner(winner: Winner, player_cases: i32, ai_cases: i32) {
    print_board(player_cases, ai_cases);

    // todo use match pattern to cover all the cases
    if matches!(winner, Winner::DRAW) {
        println!("It's a draw, not to bad...");
    }
    else if matches!(winner, Winner::PLAYER) {
        println!("You won, congratulations!");
    }
    else {
        println!("Computer won, try again :)");
    }
}

fn run() -> (Winner, i32, i32) {
    let mut player_cases = 0;
    let mut ai_cases = 0;

    let mut ai_turns: Vec<i32> = (0..9).collect();
    ai_turns.shuffle(&mut thread_rng());

    loop {
        print_board(player_cases, ai_cases);

        let player_case = prompt_player(player_cases | ai_cases);
        player_cases |= player_case;

        if (is_winner(player_cases)) {
            return (Winner::PLAYER, player_cases, ai_cases);
        }

        // check is_draw after is_winner in the case one wins on the last case
        if (is_draw(player_cases | ai_cases)) {
            return (Winner::DRAW, player_cases, ai_cases);
        }

        let ai_case = ai_turn(&mut ai_turns, player_cases | ai_cases);
        ai_cases |= ai_case;

        if (is_winner(ai_cases)) {
            return (Winner::AI, player_cases, ai_cases);
        }

        if (is_draw(player_cases | ai_cases)) {
            return (Winner::DRAW, player_cases, ai_cases);
        }
    }
}

fn is_draw(board: i32) -> bool {
    board == 0b111_111_111
}

// todo implement
fn prompt_player(board: i32) -> i32 {
    loop {
        print!("Enter a case: ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        std::io::stdin()
                .read_line(&mut buffer)
                .expect("The program failed to read your input");

        buffer.pop(); // todo remove new line depends on the system at runtime
        let case: i32 = buffer.parse().unwrap();

        if (1 << case) & board == 0 {
            return 1 << case;
        }
        else {
            println!("This case is already taken.");
        }
    }
}

fn ai_turn(ai_turns: &mut Vec<i32>, board: i32) -> i32 {
    let mut case = ai_turns.pop().unwrap();

    while (1 << case) & board != 0 {
        case = ai_turns.pop().unwrap();
    }

    println!("Computer plays case {}", case);

    1 << case
}

fn is_winner(cases: i32) -> bool {
    for s in SOLUTIONS {
        if cases & s == s {
            return true;
        }
    }

    false
}

fn print_board(player_cases: i32, ai_cases: i32) {
    println!("");
    println!("Board:");

    for case in (0..9) {
        let converted = 1 << case;

        if converted & player_cases != 0 {
            print!(" {} |", PLAYER_TOKEN);
        } else if converted & ai_cases != 0 {
            print!(" {} |", AI_TOKEN);
        } else {
            print!(" {} |", case);
        }

        if (case + 1) % 3 == 0{
            println!();
        }
    }
}
