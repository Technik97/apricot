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

impl Forth {
    pub fn new() -> Self {
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

    pub fn drop(&mut self) {
        self.pop();
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

    pub fn swap(&mut self) {
        if self.stack.len() >= 2 {
            let a = self.pop().unwrap();
            let b = self.pop().unwrap();
            self.push(a);
            self.push(b);
        }
    }

    pub fn eval(&mut self, input: &str) {
        for token in input.split_whitespace() {
            self.process_token(token);
        }
    }

    pub fn process_token(&mut self, token: &str) {
        if let Ok(number) = token.parse::<i32>() {
            self.push(number);
        } else if let Some(word) = self.dictionary.get(token).cloned() {
            self.execute_word(word);
        } else {
            eprintln!("Unknown word: {}", token);
        }
    }

    pub fn execute_word(&mut self, word: Word) {
        match word {
            Word::BuiltIn(func) => func(self),
            Word::UserDefined(tokens) => {
                for t in tokens.clone() {
                    self.eval(&t);
                }
            }
        }
    }

    pub fn define_word(&mut self, name: &str, tokens: Vec<String>) {
        self.dictionary
            .insert(name.to_string(), Word::UserDefined(tokens));
    }

    pub fn top(&self) -> Option<&i32> {
        self.stack.last()
    }
}
