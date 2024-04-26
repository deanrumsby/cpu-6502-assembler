use clap::Parser;
use std::{
    io::{stdin, stdout, Write},
    iter::Peekable,
    str::Chars,
};

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    repl: bool,

    #[arg(short, long)]
    tokens: bool,
}

#[derive(Debug)]
struct Error {
    line: usize,
    message: String,
}

fn main() {
    let cli = Cli::parse();

    if cli.repl {
        repl();
    }
}

fn repl() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => run(input),
            Err(e) => println!("Error: {e}"),
        }
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan();
    if scanner.has_error {
        println!("Error occurred");
    } else {
        println!("{tokens:?}");
    }
}

fn error(e: Error) {
    println!("Error: Line {}: {}", e.line, e.message);
}

#[derive(Clone, Debug)]
enum Token {
    Number(u32),
}

struct Scanner {
    source: String,
    line: usize,
    tokens: Vec<Token>,
    has_error: bool,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            line: 1,
            tokens: Vec::new(),
            has_error: false,
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        let mut chars = self.source.chars().peekable();
        while let Some(_) = chars.peek() {
            if let Some(t) = self.next_token(&mut chars) {
                self.tokens.push(t);
            } else {
                chars.next();
            }
        }
        self.tokens.clone()
    }

    fn next_token(&self, chars: &mut Peekable<Chars>) -> Option<Token> {
        let c = chars.peek().unwrap();

        match c {
            '$' => self.number(chars, 16),
            c => {
                if c.is_digit(10) {
                    self.number(chars, 10)
                } else {
                    None
                }
            }
        }
    }

    fn number(&self, chars: &mut Peekable<Chars>, radix: u32) -> Option<Token> {
        if radix != 10 {
            chars.next();
        }
        let mut lexeme = String::new();
        while let Some(c) = chars.peek() {
            if c.is_digit(radix) {
                lexeme.push(*c);
                chars.next();
            } else {
                break;
            }
        }
        let result = u32::from_str_radix(lexeme.as_str(), radix);
        match result {
            Ok(n) => Some(Token::Number(n)),
            Err(_) => {
                error(Error {
                    line: self.line,
                    message: "invalid number".to_string(),
                });
                None
            }
        }
    }
}
