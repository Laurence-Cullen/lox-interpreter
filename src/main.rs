mod tokens;
mod parsers;

use std::fmt::Debug;
use tokens::TokenType;
use crate::tokens::{scan_line, Token};

struct Scanner {
    source: String,
    tokens: Vec<Token>,
}

// impl Scanner {
//     fn new(source: String) -> Self {
//         Self { source , tokens: vec![] }
//     }
//
//     fn scan_tokens(&mut self) -> Vec<Token> {
//         while !self.is_at_end() {
//             let start = current;
//             scan_token()
//         }
//
//         self.tokens.push(Token::new(TokenType::Eof, String::new(), String::new(), 0));
//     }
// }

struct Lox {
    // Define the structure of the Lox interpreter
    had_error: bool,
}

impl Lox {
    fn new() -> Self {
        Self { had_error: false }
    }

    fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: u32, column: &str, message: &str) {
        eprintln!("[line {}] Error at {}: {}", line, column, message);

        self.had_error = true;
    }

    fn run_file(&mut self, path: &String) {
        let contents = std::fs::read_to_string(path).expect("Could not read file");
        Lox::run(&contents);
        if self.had_error {
            std::process::exit(65);
        }
    }

    fn run_prompt(&mut self) {
        loop {
            let mut input = String::new();
            println!("> ");
            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            if input.trim() == "exit" || input.trim() == "" {
                break;
            }
            Lox::run(&input);

            // Clear the error state after each prompt
            if self.had_error {
                self.had_error = false;
            }
        }
    }

    fn run(input: &String) {
        // let mut scanner = Scanner::new(input.clone());
        let tokens = scan_line(input);

        for token in tokens {
            // Print or process the token
            println!("{:?}", token);
        }
        println!("Bye!");
    }
}



fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let mut lox = Lox::new();

    println!("{:?}", args);

    if args.len() == 2 {
        lox.run_file(&args[1]);
    } else if args.len() == 1 {
        lox.run_prompt();
    } else {
        println!("Usage: rlox [path]");
        // system exit 64
        std::process::exit(64);
    }
}
