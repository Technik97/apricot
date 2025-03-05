use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Word {
    BuiltIn(fn(&mut Forth)),
    UserDefined(Vec<String>),
}

pub struct Forth {
    stack: Vec<i32>,
    dictionary: HashMap<String, Word>,
}

macro_rules! define_builtins {
    ($($name:expr => $func:ident),* $(,)?) => {
        vec![
            $(($name.to_string(), Word::BuiltIn(|forth| forth.$func()))),*
        ]
    };
}

lazy_static! {
    static ref BUILTIN_WORDS: HashMap<String, Word> = {
        let builtins = define_builtins! {
            "+" => add,
            "-" => sub,
            "*" => mul,
            "/" => div,
            "dup" => dup,
            "drop" => drop_word,  // Renamed to `drop_word` to avoid name conflict
            "swap" => swap,
            "over" => over,
        };

        builtins.into_iter().collect()
    };
}

impl Forth {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            dictionary: BUILTIN_WORDS.clone(),
        }
    }

    /// Defines a new user word
    pub fn define_word(&mut self, name: &str, tokens: Vec<String>) {
        self.dictionary.insert(name.to_string(), Word::UserDefined(tokens));
    }

    /// Evaluates input and executes built-in or user-defined words
    pub fn eval(&mut self, input: &str) {
        let words: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

        for word in words {
            if let Some(word_def) = self.dictionary.get(&word).cloned() {
                match word_def {
                    Word::BuiltIn(func) => func(self),
                    Word::UserDefined(tokens) => {
                        for token in tokens {
                            self.eval(&token);  // Recursive evaluation
                        }
                    }
                }
            } else if let Ok(num) = word.parse::<i32>() {
                self.push(num);
            } else {
                eprintln!("Unknown word: {}", word);
            }
        }
    }

    /// Returns the top element of the stack
    pub fn top(&self) -> Option<i32> {
        self.stack.last().copied()
    }

    pub fn add(&mut self) {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            self.push(a + b);
        }
    }

    pub fn sub(&mut self) {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            self.push(b - a);
        }
    }

    pub fn mul(&mut self) {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            self.push(a * b);
        }
    }

    pub fn div(&mut self) {
        if let (Some(a), Some(b)) = (self.pop(), self.pop()) {
            if a != 0 {
                self.push(b / a);
            } else {
                eprintln!("Error: Division by zero");
            }
        }
    }

    pub fn drop_word(&mut self) {
        self.pop();
    }

    pub fn swap(&mut self) {
        if self.stack.len() >= 2 {
            let len = self.stack.len();
            self.stack.swap(len - 1, len - 2);
        }
    }

    pub fn over(&mut self) {
        if self.stack.len() >= 2 {
            let a = self.stack[self.stack.len() - 2];
            self.push(a);
        }
    }

    pub fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }

    pub fn dup(&mut self) {
        if let Some(&top) = self.stack.last() {
            self.push(top);
        }
    }
}
