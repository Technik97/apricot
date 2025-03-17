use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::builtins;
use crate::word::Word;

pub struct Forth {
    pub stack: Vec<i32>,
    pub dictionary: HashMap<String, Word>,
}

/// Trait for built-in operations
pub trait BuiltinOps {
    fn add(&mut self);
    fn sub(&mut self);
    fn mul(&mut self);
    fn div(&mut self);
    fn dup(&mut self);
    fn drop_word(&mut self);
    fn swap(&mut self);
    fn over(&mut self);
}

/// Implementation of BuiltinOps for Forth
impl BuiltinOps for Forth {
    fn add(&mut self) { builtins::add(self); }
    fn sub(&mut self) { builtins::sub(self); }
    fn mul(&mut self) { builtins::mul(self); }
    fn div(&mut self) { builtins::div(self); }
    fn dup(&mut self) { builtins::dup(self); }
    fn drop_word(&mut self) { builtins::drop_word(self); }
    fn swap(&mut self) { builtins::swap(self); }
    fn over(&mut self) { builtins::over(self); }
}

/// Macro to define built-in words
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
            "drop" => drop_word,
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

    /// Defines a new user-defined word
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
                            self.eval(&token);
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

    pub fn push(&mut self, value: i32) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.stack.pop()
    }
}
