use clap::Parser;
use phf::phf_map;
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
    Identifier(String),
    Op(Op),
}

#[derive(Clone, Debug)]
enum Op {
    ADC,
}

static OPS: phf::Map<&'static str, Op> = phf_map! {
    "adc" => Op::ADC,
};

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
            match self.next_token(&mut chars) {
                Some(res) => match res {
                    Ok(t) => self.tokens.push(t),
                    Err(e) => {
                        self.has_error = true;
                        error(e);
                    }
                },
                None => {
                    chars.next();
                }
            }
        }
        self.tokens.clone()
    }

    fn next_token(&self, chars: &mut Peekable<Chars>) -> Option<Result<Token, Error>> {
        let c = chars.peek().unwrap();

        match c {
            '$' => Some(self.number(chars, 16)),
            c => {
                if c.is_digit(10) {
                    Some(self.number(chars, 10))
                } else if c.is_alphabetic() {
                    Some(self.identifier(chars))
                } else {
                    None
                }
            }
        }
    }

    fn number(&self, chars: &mut Peekable<Chars>, radix: u32) -> Result<Token, Error> {
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
            Ok(n) => Ok(Token::Number(n)),
            Err(_) => Err(Error {
                line: self.line,
                message: "invalid number".to_string(),
            }),
        }
    }

    fn identifier(&self, chars: &mut Peekable<Chars>) -> Result<Token, Error> {
        let mut lexeme = String::new();

        while let Some(c) = chars.peek() {
            if c.is_alphanumeric() {
                lexeme.push(*c);
                chars.next();
            } else {
                break;
            }
        }

        if let Some(op) = OPS.get(lexeme.to_lowercase().as_str()) {
            return Ok(Token::Op(op.clone()));
        }

        Ok(Token::Identifier(lexeme))
    }
}
