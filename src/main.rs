//TODO: ADD DOCUMENTATION

use std::io;
use std::str::FromStr;
use std::io::Write;

const OPERATORS: &str =
    "Available Operators:\n\
    +   Addition\n\
    -   Subtraction\n\
    *   Multiplication\n\
    /   Division\n";


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
    fn do_math(&self, symbol: char) {
        match symbol {
            '+' => {
                self.display_math(symbol, self.add());
            }
            '-' => {
                self.display_math(symbol, self.subtract());
            }
            '*' => {
                self.display_math(symbol, self.multiply());
            }
            '/' => {
                self.display_math(symbol, self.divide());
            }
            _ => (),
        }
    }
}

fn input<T: FromStr>() -> T {
    let mut output = String::new();
    io::stdin().read_line(&mut output).unwrap();
    auto_parse(output)
}

fn auto_parse<T: FromStr>(input: String) -> T {
    let output: T = input.trim().parse().unwrap_or_else(|_| {
        eprintln!("Can you input whats asked? Thanks.");
        panic!();
    });
    output
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        print!("Welcome to ZigTag's calculator\n\
            Please enter a symbol: ");
        io::stdout().flush().unwrap();
        let mut symbol: char;

        loop {
            symbol = input();

            if !((symbol == '+') || (symbol == '-') || (symbol == '*') || (symbol == '/')) {
                print!("{}", OPERATORS);
            } else {
                break;
            }

            print!("\nPlease enter a symbol: ");
            io::stdout().flush().unwrap();
        }

        print!("Now enter your first number: ");
        io::stdout().flush().unwrap();
        let first_number: f64 = input();

        print!("Now enter your second number: ");
        io::stdout().flush().unwrap();
        let second_number: f64 = input();

        let numbers = Numbers::new(first_number, second_number);

        numbers.do_math(symbol);
    }
    else if args.len() == 2 {
        if args[1] == "-h" || args[1] == "--help" {
            println!("ZigTag's Calculator\n\
                Usage: calculator <first_number> <second_number> <symbol>\n\
                {}", OPERATORS
            )
        }
    }
    else if args.len() <= 3 {
        println!("You need 3 arguments.");
    }
    else if args.len() == 4 {
        let first_number: f64 = auto_parse(args[1].clone());
        let second_number: f64 = auto_parse(args[2].clone());
        let symbol: char = auto_parse(args[3].clone());

        let numbers = Numbers::new(first_number, second_number);

        numbers.do_math(symbol);
    }
    else {
        println!("Too many arguments.")
    }


}