use clipboard::{ClipboardContext, ClipboardProvider};
use rand::{thread_rng, Rng};
use std::{env::args, ops::RangeInclusive, process};

fn get_symbol() -> char {
    let symbols = "!@#$%^&*";

    symbols
        .chars()
        .nth(thread_rng().gen_range(0..symbols.len() - 1))
        .unwrap()
}

fn get_char_from_range(range: RangeInclusive<u32>) -> char {
    char::from_u32(thread_rng().gen_range(range)).unwrap()
}

fn passgen(length: u32) -> String {
    let mut password = String::new();
    let types = ["number", "lowercase", "uppercase", "symbols"];

    for _ in 0..length {
        let char_type = types[thread_rng().gen_range(0..4)];

        match char_type {
            "number" => password.push(get_char_from_range(48..=57)),
            "lowercase" => password.push(get_char_from_range(97..=122)),
            "uppercase" => password.push(get_char_from_range(65..=90)),
            "symbols" => password.push(get_symbol()),
            _ => continue,
        }
    }

    password
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        eprintln!("Length needs to be a positive number");
        eprintln!("Usage: passgen <length>");
        process::exit(1);
    }

    let mut password_length = 20;

    if args.len() > 1 {
        password_length = match args[1].parse() {
            Ok(length) => {
                if length > 128 || length < 8 {
                    eprintln!("Length needs to be between 8 and 128");
                    process::exit(1);
                }

                length
            }
            Err(_) => {
                eprintln!("Length needs to be a positive number");
                eprintln!("Usage: passgen <length>");
                process::exit(1);
            }
        };
    }

    let password = passgen(password_length);
    println!("Password: {}", password);

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    ctx.set_contents(password).unwrap();
    ctx.get_contents().unwrap();

    println!("Copied to clipboard");
}
