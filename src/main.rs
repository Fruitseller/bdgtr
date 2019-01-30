use std::env;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
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
                let file = get_file("expenses.csv");
                let expenses = parse_expenses(&file);
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
            }
        }
        _ => panic!("Too many arguments!"),
    }
}

#[derive(Debug, PartialEq)]
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
    match OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open(path)
    {
        Ok(file) => file,
        Err(_) => {
            println!("Could not read file at: {}", path);
            process::exit(1);
        }
    }
}

fn parse_expenses(file: &File) -> Vec<Expense> {
    let reader = BufReader::new(file);
    let mut expenses: Vec<Expense> = Vec::new();
    for result in reader.lines() {
        let line = match result {
            Ok(l) => l,
            Err(_) => continue,
        };
        let mut split_result: Vec<String> = line.split(",").map(|s| s.to_string()).collect();

        if split_result.len() == 2 {
            let parse_result = split_result.remove(1).parse::<f64>();
            let amount = match parse_result {
                Ok(a) => a,
                Err(_) => continue,
            };
            let name = split_result.remove(0);
            let expense = Expense { name, amount };
            expenses.push(expense);
        }
    }

    expenses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let expense = Expense {
            name: "foo".to_string(),
            amount: 22.3,
        };
        assert_eq!("foo,22.3", expense.to_string());
    }

    #[test]
    fn test_parse_expenses() {
        let file = get_file("src/happy_path.csv");
        let actual_expenses = parse_expenses(&file);
        let expected_expenses = vec![
            Expense {
                name: "netflix".to_string(),
                amount: 42f64,
            },
            Expense {
                name: "google".to_string(),
                amount: 3f64,
            },
        ];

        assert_eq!(expected_expenses, actual_expenses);
    }
}
