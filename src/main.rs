use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();

    loop {
        print!(">> ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");

        for c in line.chars() {
            match c {
                '(' => println!("{}", c),
                ')' => println!("{}", c),
                '+' => { x + y; },
                '-' => { x - y; },
                '*' => { x * y; },
                '/' => { x / y; },
                _ => {
                    if c.is_numeric() {
                    }
                }
            }
        }
    }
}
