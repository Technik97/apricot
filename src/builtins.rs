use crate::forth::Forth;

pub fn add(forth: &mut Forth) {
    if let (Some(a), Some(b)) = (forth.pop(), forth.pop()) {
        forth.push(a + b);
    }
}

pub fn sub(forth: &mut Forth) {
    if let (Some(a), Some(b)) = (forth.pop(), forth.pop()) {
        forth.push(b - a);
    }
}

pub fn mul(forth: &mut Forth) {
    if let (Some(a), Some(b)) = (forth.pop(), forth.pop()) {
        forth.push(a * b);
    }
}

pub fn div(forth: &mut Forth) {
    if let (Some(a), Some(b)) = (forth.pop(), forth.pop()) {
        if a != 0 {
            forth.push(b / a);
        } else {
            eprintln!("Error: Division by zero");
        }
    }
}

pub fn dup(forth: &mut Forth) {
    if let Some(&top) = forth.stack.last() {
        forth.push(top);
    }
}

pub fn drop_word(forth: &mut Forth) {
    forth.pop();
}

pub fn swap(forth: &mut Forth) {
    if forth.stack.len() >= 2 {
        let len = forth.stack.len();
        forth.stack.swap(len - 1, len - 2);
    }
}

pub fn over(forth: &mut Forth) {
    if forth.stack.len() >= 2 {
        let a = forth.stack[forth.stack.len() - 2];
        forth.push(a);
    }
}
