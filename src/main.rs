use std::io::stdin;
use rusty_budget::budget::Budget;

fn init_budget() -> Budget {
    let mut name: String = String::new();
    let mut desc: String = String::new();

    println!("Please provide a name for this budget tracker:");
    match stdin().read_line(&mut name) {
        Ok(_) => println!("Thanks."),
        Err(err) => println!("Error: {err}"),
    }

    println!("Please provide a description for this budget tracker:");
    match stdin().read_line(&mut desc) {
        Ok(_) => println!("Thanks."),
        Err(err) => println!("Error: {err}"),
    }

    Budget::new(name, desc)
}

fn main() {
    let mut budget = init_budget();
    budget.add_expense();
}