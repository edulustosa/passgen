use copypasta::{ClipboardContext, ClipboardProvider};
use rand::{thread_rng, Rng};
use std::env::args;

fn get_symbol() -> char {
    let symbols = String::from("!@#$%¨&*+*/:~|=-");
    symbols
        .chars()
        .nth(thread_rng().gen_range(0..symbols.len() - 1))
        .unwrap()
}

fn passgen(length: u32) -> String {
    let mut password = String::new();
    let types = ["number", "lowercase", "uppercase", "symbols"];

    for _i in 0..length {
        let char_type = types[thread_rng().gen_range(0..4)];

        match char_type {
            "number" => password.push(char::from_u32(thread_rng().gen_range(48..=57)).unwrap()),
            "lowercase" => password.push(char::from_u32(thread_rng().gen_range(97..=122)).unwrap()),
            "uppercase" => password.push(char::from_u32(thread_rng().gen_range(65..=90)).unwrap()),
            "symbols" => password.push(get_symbol()),
            _ => continue,
        }
    }

    password
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("Too many arguments");
        println!("Usage: passgen <length>");
        return;
    }

    let mut password_length = 20;

    if args.len() > 1 {
        password_length = match args[1].parse() {
            Ok(x) => x,
            Err(_) => {
                println!("Length needs to be a number");
                println!("Usage: passgen <length>");
                return;
            }
        };
    }

    let password = passgen(password_length);
    println!("Password: {}", password);

    let mut ctx = ClipboardContext::new().unwrap();

    ctx.set_contents(password).unwrap();
    ctx.get_contents().unwrap();

    println!("Copied to clipboard");
}
