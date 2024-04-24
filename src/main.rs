use clap::Parser;
use std::io::{stdin, stdout, Write};

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
    Op,
}

struct Scanner {
    source: String,
    start: u32,
    current: u32,
    line: u32,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        self.tokens.clone()
    }
}
