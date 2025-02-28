use std::collections::HashMap;
use std::io::{self, Write};

enum Word {
    BuiltIn(fn(&mut Forth)),
    UserDefined(Vec<String>),
}

struct Forth {
    stack: Vec<i32>,
    dictionary: HashMap<String, Word>,
}

impl Forth {
    fn new() -> Self {
        let mut dictionary = HashMap::new();
        dictionary.insert("+".to_string(), Word::BuiltIn(Forth::add));
        dictionary.insert("dup".to_string(), Word::BuiltIn(Forth::dup));
        dictionary.insert("-".to_string(), Word::BuiltIn(Forth::sub));
        dictionary.insert("*".to_string(), Word::BuiltIn(Forth::mul));
        dictionary.insert("/".to_string(), Word::BuiltIn(Forth::div));
        dictionary.insert("drop".to_string(), Word::BuiltIn(Forth::drop));
        dictionary.insert("over".to_string(), Word::BuiltIn(Forth::over));

        Self {
            stack: Vec::new(),
            dictionary,
        }
    }

    fn add(&mut self) {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            self.push(a + b);
        }
    }

    fn sub(&mut self) {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            self.push(b - a);
        }
    }

    fn mul(&mut self) {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            self.push(a * b);
        }
    }

    fn div(&mut self) {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            if a != 0 {
                self.push(b / a);
            } else {
                eprintln!("Error: Division by zero");
            }
        }
    }

    fn drop(&mut self) {
        self.pop();
    }

    fn over(&mut self) {
        if self.stack.len() >= 2 {
            let a = self.stack[self.stack.len() - 2];
            self.push(a);
        }
    }

    fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    fn dup(&mut self) {
        if let Some(&top) = self.stack.last() {
            self.push(top);
        }
    }

    fn swap(&mut self) {
        if self.stack.len() >= 2 {
            let a = self.pop().unwrap();
            let b = self.pop().unwrap();
            self.push(a);
            self.push(b);
        }
    }

    fn eval(&mut self, input: &str) {
        for token in input.split_whitespace() {
            if let Ok(number) = token.parse::<i32>() {
                self.push(number);
            } else if let Some(word) = self.dictionary.get(token) {
                match word {
                    Word::BuiltIn(func) => func(self),
                    Word::UserDefined(tokens) => {
                        for t in tokens.clone() {
                            self.eval(&t);
                        }
                    }
                }
            } else {
                eprintln!("Unknown word: {}", token);
            }
        }
    }

    fn define_word(&mut self, name: &str, tokens: Vec<String>) {
        self.dictionary
            .insert(name.to_string(), Word::UserDefined(tokens));
    }

    fn top(&self) -> Option<&i32> {
        self.stack.last()
    }
}

fn main() {
    let mut forth = Forth::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "exit" {
            break;
        }

        forth.eval(&input);

        if let Some(top) = forth.top() {
            println! {"{}", top};
        } else {
            println!("Stack is empty");
        }
    }
}
