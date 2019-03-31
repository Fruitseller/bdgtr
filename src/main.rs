use std::env;
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::process;

const EXPENSE_NAME_HEADER: &str = "What";
const EXPENSE_AMOUNT_HEADER: &str = "How much";

type Result<T> = std::result::Result<T, ExpenseError>;

#[derive(Debug, Clone)]
struct ExpenseError {
    message: String,
}

impl ExpenseError {
    pub fn new(m: &str) -> ExpenseError {
        ExpenseError {
            message: m.to_string(),
        }
    }
}

impl fmt::Display for ExpenseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

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
                let file = get_file_for_read("expenses.csv");
                let expenses = parse_expenses(&file);
                print_expense_table(&expenses);
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
                let mut file = get_file_for_write("expenses.csv");
                writeln!(file, "{}", expense.to_string());
            }
        }
        _ => panic!("Too many arguments!"),
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Expense {
    name: String,
    amount: f64,
}

struct PaddedExpense {
    name: String,
    amount: String,
}

impl fmt::Display for Expense {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.name)?;
        f.write_str(",")?;
        f.write_str(&self.amount.to_string())?;
        Ok(())
    }
}

fn get_file_for_read(path: &str) -> File {
    match OpenOptions::new().read(true).open(path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Could not read file at: {}", path);
            process::exit(1);
        }
    }
}

fn get_file_for_write(path: &str) -> File {
    match OpenOptions::new().append(true).create(true).open(path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Could not read file at: {}", path);
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

fn print_expense_table(expenses: &[Expense]) {
    let longest_name = find_longest_expense_name(expenses);
    let longest_amount = find_longest_expense_amount(expenses);

    // TODO use ? operator instead of unwrap()
    let padded_expenses = pad_expenses(expenses, longest_name, longest_amount).unwrap();

    print_header(longest_name, longest_amount);
    for expense in padded_expenses {
        println!("| {} | {} |", expense.name, expense.amount);
    }
    print_footer(longest_name, longest_amount);
}

fn print_header(longest_name: i32, longest_amount: i32) {
    let name_border = (0..longest_name + 2).map(|_| "-").collect::<String>();
    let amount_border = (0..longest_amount + 2).map(|_| "-").collect::<String>();
    println!("+{}+{}+", name_border, amount_border);

    if longest_name > EXPENSE_NAME_HEADER.len() as i32
        || longest_amount > EXPENSE_AMOUNT_HEADER.len() as i32
    {
        let name_difference = longest_name - EXPENSE_NAME_HEADER.len() as i32;
        let name_spaces = (0..name_difference).map(|_| " ").collect::<String>();
        let amount_difference = longest_amount - EXPENSE_AMOUNT_HEADER.len() as i32;
        let amount_spaces = (0..amount_difference).map(|_| " ").collect::<String>();
        println!(
            "| {} | {} |",
            format!("{}{}", EXPENSE_NAME_HEADER, name_spaces).to_string(),
            format!("{}{}", EXPENSE_AMOUNT_HEADER, amount_spaces).to_string()
        );
    } else {
        println!("| What | How much |");
    }

    println!("+{}+{}+", name_border, amount_border);
}

fn print_footer(longest_name: i32, longest_amount: i32) {
    let name_border = (0..longest_name + 2).map(|_| "-").collect::<String>();
    let amount_border = (0..longest_amount + 2).map(|_| "-").collect::<String>();
    println!("+{}+{}+", name_border, amount_border);
}

fn find_longest_expense_name(expenses: &[Expense]) -> i32 {
    let mut longest: i32 = EXPENSE_NAME_HEADER.len() as i32;
    for expense in expenses {
        let actual_length = expense.name.len() as i32;
        if actual_length > longest {
            longest = actual_length;
        }
    }
    longest
}

fn find_longest_expense_amount(expenses: &[Expense]) -> i32 {
    let mut longest = EXPENSE_AMOUNT_HEADER.len() as i32;
    for expense in expenses {
        let actual_length = expense.amount.to_string().len() as i32;
        if actual_length > longest {
            longest = actual_length;
        }
    }

    longest
}

fn pad_expenses(
    expenses: &[Expense],
    longest_name: i32,
    longest_amount: i32,
) -> Result<Vec<PaddedExpense>> {
    let mut padded_expenses = Vec::new();
    for expense in expenses {
        let actual_name_length = expense.name.len() as i32;
        let actual_amount_length = expense.amount.to_string().len() as i32;
        if actual_name_length == longest_name && actual_amount_length == longest_amount {
            padded_expenses.push(PaddedExpense {
                name: expense.name.to_string(),
                amount: expense.amount.to_string(),
            });
        } else {
            let name_difference = longest_name - actual_name_length;
            let amount_difference = longest_amount - actual_amount_length;
            if name_difference < 0 || amount_difference < 0 {
                let error: ExpenseError = ExpenseError::new("Padding went wrong!");
                return Err(error);
            }
            let name_spaces = (0..name_difference).map(|_| " ").collect::<String>();
            let amount_spaces = (0..amount_difference).map(|_| " ").collect::<String>();
            padded_expenses.push(PaddedExpense {
                name: format!("{}{}", expense.name, name_spaces).to_string(),
                amount: format!("{}{}", expense.amount.to_string(), amount_spaces).to_string(),
            });
        }
    }

    Ok(padded_expenses)
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
        let file = get_file_for_read("test_files/happy_path.csv");
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
        let file = get_file_for_read("test_files/broken_entries.csv");
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
    fn pad_expense_names_happy_path() {
        let expenses = vec![Expense {
            name: "foo".to_string(),
            amount: 0f64,
        }];
        let result_expenses = pad_expenses(&expenses, 4, 8);
        assert_eq!(true, result_expenses.is_ok());
    }

    #[test]
    fn pad_expense_names_broken() {
        let expenses = vec![Expense {
            name: "foo".to_string(),
            amount: 0f64,
        }];
        let result_expenses = pad_expenses(&expenses, 2, 3);
        assert_eq!(true, result_expenses.is_err());
    }
}
