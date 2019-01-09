use std::env;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::Write;
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
                let mut file = get_file("expenses.csv");
                writeln!(file, "{}", expense.to_string());
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

impl fmt::Display for Expense {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name)?;
        f.write_str(",")?;
        f.write_str(&self.amount.to_string())?;
        Ok(())
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let expense = Expense {name: "foo".to_string(), amount: 22.3};
        assert_eq!("foo,22.3", expense.to_string());
    }
}
