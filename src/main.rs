use render::{print_expense_summary, print_expense_table};
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process;
mod render;
use io::{get_file_for_read, get_file_for_write};
mod io;

const EXPENSES: &str = "expenses";
const ADD: &str = "add";

#[derive(Debug, PartialEq, Clone)]
pub struct Expense {
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

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("I am groot");
        }
        2 => {
            if args[1].ne(EXPENSES) {
                println!("Wrong argument");
            } else {
                match get_file_for_read("expenses.csv") {
                    Ok(file) => {
                        let expenses = parse_expenses(&file);
                        print_expense_table(&expenses);
                        print_expense_summary(&expenses);
                    }
                    Err(e) => {
                        eprintln!("Could not read file: {:#?}", e);
                        process::exit(1)
                    }
                };
            }
        }
        4 => {
            if args[1].eq(ADD) {
                let result = args[3].parse::<f64>();
                let amount = match result {
                    Ok(amount) => amount,
                    Err(e) => panic!(e),
                };
                let name = args[2].to_string();
                let expense = Expense { name, amount };
                let mut file = match get_file_for_write("expenses.csv") {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("Could not read file: {:#?}", e);
                        process::exit(1)
                    }
                };
                match writeln!(file, "{}", expense.to_string()) {
                    Ok(_) => "Successfully added expense",
                    Err(e) => panic!(e),
                };
            }
        }
        _ => panic!("Too many arguments!"),
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
        let mut split_result: Vec<String> = line.split(',').map(|s| s.to_string()).collect();

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
    fn parse_expenses_with_happy_path() {
        let file = get_file_for_read("test_files/happy_path.csv").unwrap();
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

    #[test]
    fn parse_expenses_with_broken_entries() {
        let file = get_file_for_read("test_files/broken_entries.csv").unwrap();
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
