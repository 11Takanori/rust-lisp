use std::io::{self, BufRead, Write};
use std::env;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct LRef(i64);

#[derive(Debug)]
struct Arena {
    last: i64,
    data: HashMap<LRef, LObj>,
}

impl Arena {
    fn new() -> Arena {
        Arena {
            last: 0,
            data: HashMap::new(),
        }
    }

    fn get(&self, key: &LRef) -> LObj {
        match self.data.get(&key) {
            Some(val) => val.clone(),
            _ => LObj::Nil,
        }
    }

    fn set(&mut self, key: LRef, val: LObj) {
        self.data.insert(key, val);
    }

    fn make(&mut self, obj: LObj) -> LRef {
        self.last += 1;
        self.data.insert(LRef(self.last), obj);
        LRef(self.last)
    }
}

#[derive(Clone, PartialEq, Debug)]
enum LObj {
    Nil,
    Sym(String),
    Num(i64),
    Subr(SubFn),
    Expr(LRef, LRef),
    Cons(LRef, LRef),
}

#[derive(Clone, PartialEq, Debug)]
enum SubFn {
    Car,
    Cdr,
    Cons,
    Eq,
    Atom,
    Numberp,
    Symbolp,
    Add,
    Mul,
    Sub,
    Div,
    Mod,
    T,
}

#[derive(Debug)]
struct Evaluator {
    arena: Arena,
    genv: LRef,
}

fn read(str: String) -> String {
    str
}

fn eval(c: String, arena: &mut Arena) -> i32 {
    0
}

fn main() {
    let stdin = io::stdin();
    let mut arena = Arena::new();

    loop {
        print!(">> ");
        io::stdout().flush().expect("Error flushing stdout");

        let mut line = String::new();
        stdin.lock().read_line(&mut line).expect("Error reading from stdin");

        // println!("{}", read(line));
        eval(line, &mut arena);
    }
}

