use super::Expense;
use std::fmt;

const EXPENSE_NAME_HEADER: &str = "What";
const EXPENSE_AMOUNT_HEADER: &str = "How much";
const EXPENSE_SUMMARY: &str = "Summary";

struct PaddedExpense {
    name: String,
    amount: String,
}

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

pub fn print_expense_table(expenses: &[Expense]) {
    let longest_name = find_longest_expense_name(expenses);
    let longest_amount = find_longest_expense_amount(expenses);

    let padded_expenses = pad_expenses(expenses, longest_name, longest_amount).unwrap();

    print_header(longest_name, longest_amount);
    for expense in padded_expenses {
        println!("| {} | {} |", expense.name, expense.amount);
    }
    print_spacer(longest_name, longest_amount);
}

fn print_header(longest_name: i32, longest_amount: i32) {
    print_spacer(longest_name, longest_amount);

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
        println!(
            "{}",
            format!("| {} | {} |", EXPENSE_NAME_HEADER, EXPENSE_AMOUNT_HEADER)
        );
    }

    print_spacer(longest_name, longest_amount);
}

fn print_spacer(longest_name: i32, longest_amount: i32) {
    let name_border = (0..longest_name + 2).map(|_| "-").collect::<String>();
    let amount_border = (0..longest_amount + 2).map(|_| "-").collect::<String>();
    println!("+{}+{}+", name_border, amount_border);
}

pub fn print_expense_summary(expenses: &[Expense]) {
    let mut sum: f64 = 0.0;
    for expense in expenses {
        sum += expense.amount;
    }
    println!(" {}: {} €", EXPENSE_SUMMARY, sum.to_string());
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
