struct Forth {
    stack: Vec<i32>,
}

impl Forth {
    fn new() -> Self {
        Self { stack: Vec::new() }
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
}

fn main() {
    let mut stack = Forth::new();
    stack.push(5);
}
