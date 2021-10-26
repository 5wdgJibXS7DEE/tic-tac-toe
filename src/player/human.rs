use std::io::{self, Write};

pub struct Human {
    symbol: char,
}

impl Human {
    pub fn new(symbol: char) -> Human {
        Human { symbol: symbol }
    }
}

impl super::Play for Human {
    fn get_symbol(&self) -> char {
        self.symbol
    }

    fn play(&mut self) -> u16 {
        print!("Your turn {}: ", self.symbol);
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .expect("The program failed to read your input");

        buffer.pop(); // todo remove new line depends on the system at runtime
        
        let parsed: u16 = match &buffer[..] {
            // todo not all possibilities
            "0" | "00" => 0,
            "1" | "01" => 1,
            "2" | "02" => 2,
            "3" | "03" => 3,
            "4" | "04" => 4,
            "5" | "05" => 5,
            "6" | "06" => 6,
            "7" | "07" => 7,
            "8" | "08" => 8,
            "9" | "09" => 9,
            "a" | "A" | "0a" | "0A" => 10,
            "b" | "B" | "0b" | "0B" => 11,
            "c" | "C" | "0c" | "0C" => 12,
            "d" | "D" | "0d" | "0D" => 13,
            "e" | "E" | "0e" | "0E" => 14,
            "f" | "F" | "0f" | "0F" => 15,
            _ => 0,
        };

        parsed
    }
}
