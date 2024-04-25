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
            Ok(_) => {}
            Err(e) => println!("Error: {e}"),
        }

        run(input);
    }
}

fn run(source: String) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan();
    println!("{tokens:?}");
}

#[derive(Clone, Debug)]
enum Token {
    Number(u32),
}

struct Scanner {
    source: String,
    line: usize,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            line: 1,
            tokens: Vec::new(),
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
            _ => None,
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
        Some(Token::Number(
            u32::from_str_radix(lexeme.as_str(), radix).expect("invalid token"),
        ))
    }
}
