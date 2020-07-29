use std::io;
use std::str::FromStr;
use std::io::Write;

struct Numbers {
    x: f64,
    y: f64,
}

impl Numbers {
    fn new(x: f64, y: f64) -> Self {
        Numbers {x, y}
    }
    fn add(&self) -> f64 {
        self.x + self.y
    }
    fn subtract(&self) -> f64 {
        self.x - self.y
    }
    fn multiply(&self) -> f64 {
        self.x * self.y
    }
    fn divide(&self) -> f64 {
        self.x / self.y
    }
    fn display_math(&self, symbol: char, result: f64) {
        println!("{} {} {} = {}", self.x, symbol, self.y, result)
    }
}

fn input<T: FromStr>() -> T {
    let mut output = String::new();
    io::stdin().read_line(&mut output).unwrap();
    let output: T = output.trim().parse().unwrap_or_else(|_| {
        eprintln!("Can you input whats asked? Thanks.");
        panic!();
    });
    output
}

fn main() {
    print!("Welcome to ZigTag's calculator\n\
              Please enter a symbol: ");
    io::stdout().flush().unwrap();
    let symbol: char = input();

    print!("Now enter your first number: ");
    io::stdout().flush().unwrap();
    let first_number: f64 = input();

    print!("Now enter your second number: ");
    io::stdout().flush().unwrap();
    let second_number: f64 = input();

    let numbers = Numbers::new(first_number, second_number);

    match symbol {
        '+' => {
            numbers.display_math(symbol, numbers.add());
        }
        '-' => {
            numbers.display_math(symbol, numbers.subtract());
        }
        '*' => {
            numbers.display_math(symbol, numbers.multiply());
        }
        '/' => {
            numbers.display_math(symbol, numbers.divide());
        }
        _ => (),
    }
}