use apricot::forth::Forth;
use std::io::{self, Write};

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

        if input.starts_with(':') {
            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.len() >= 3 && parts.last() == Some(&";") {
                let name = parts[1].to_string();
                let tokens = parts[2..parts.len() - 1]
                    .iter()
                    .map(|s| s.to_string())
                    .collect();
                forth.define_word(&name, tokens);
                continue;
            } else {
                eprintln!("Invalid word definition");
                continue;
            }
        }

        forth.eval(input);

        if let Some(top) = forth.top() {
            println! {"{}", top};
        } else {
            println!("Stack is empty");
        }
    }
}
