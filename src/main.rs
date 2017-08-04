use std::io::{self, BufRead, Write};
use std::env;

fn main() {
    // let stdin = io::stdin();

    // loop {
    //     print!(">> ");
    //     io::stdout().flush().expect("Error flushing stdout");

    //     let mut line = String::new();
    //     stdin.lock().read_line(&mut line).expect("Error reading from stdin");

    //     println!("{}", eval(read(line)));
    // }

    let line =  env::args().nth(1).unwrap();
    for c in line.trim().chars() {
        // eval(c);
        // println!("{}", c);
        // println!("{}", eval(c));
    }
    // read(line);
}


fn read(str: String) -> String {
    println!("{:?}", str);
    str
}

fn eval(c: char) -> i32 {
    if c.is_numeric() {

    }

    let x = 1;
    let y = 2;

    match c {
            // '(' => println!("{}", c),
            // ')' => println!("{}", c),
            '+' => { return x + y; },
            '-' => { return x - y; },
            '*' => { return x * y; },
            '/' => { return x / y; },
            _ => {
            }
    }
    // println!("{:?}", c);
    0
}

fn eval2(line: String) -> i32 {
    for c in line.chars() {
        if c.is_numeric() {
        }

        let x = 1;
        let y = 2;
        match c {
            // '(' => println!("{}", c),
            // ')' => println!("{}", c),
            '+' => { return x + y; },
            '-' => { return x - y; },
            '*' => { return x * y; },
            '/' => { return x / y; },
            _ => {
            }
        }
    }
    0
}