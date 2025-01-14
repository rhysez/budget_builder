use crate::budget::Budget;
use std::io::stdin;

pub fn init_budget() -> Budget {
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
