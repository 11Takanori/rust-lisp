use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();

    loop {
        print!(">> ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");

        println!("{}", eval(read(line)));
    }
}


fn read(str: String) -> String {
    str
}

fn eval(line: String) -> String {

    for c in line.chars() {
        if c.is_numeric() {

        }

        match c {
            // '(' => println!("{}", c),
            // ')' => println!("{}", c),
            '+' => { x + y; },
            '-' => { x - y; },
            '*' => { x * y; },
            '/' => { x / y; },
            _ => {
            }
        }
    }
    line
}