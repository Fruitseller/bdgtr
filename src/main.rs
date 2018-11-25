use std::env;
use std::fs::{File, OpenOptions};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("I am groot");
        }
        2 => {
            if args[1] != "expenses" {
                println!("Wrong argument");
            } else {
                print_expenses();
            }
        }
        4 => {
            if args[1] == "add" {
                let result = args[3].parse::<f64>();
                let amount = match result {
                    Ok(amount) => amount,
                    Err(e) => panic!(e),
                };
                let name = args[2].to_string();
                let expense = Expense { name, amount };
                let file = get_file("expenses.json");
                println!("{:?}", file);
            }
        }
        _ => panic!("Too many arguments!"),
    }
}

#[derive(Debug)]
struct Expense {
    name: String,
    amount: f64,
}

fn get_file(path: &str) -> File {
    match OpenOptions::new().append(true).create(true).open(path) {
        Ok(file) => file,
        Err(_) => {
            println!("Could not read file at: {}", path);
            process::exit(1);
        }
    }
}

fn print_expenses() {}
