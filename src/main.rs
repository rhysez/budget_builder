use std::collections::HashMap;
use std::io::{stdin, stdout};

struct Budget {
    name: String,
    desc: String,
    items: HashMap<String, u32>,
}

impl Budget {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_desc(&self) -> &String {
        &self.desc
    }

    fn get_expenses(&self) {
        for (name, cost) in &self.items {
            println!("{name}: {cost}")
        }
    }
}

impl Budget {
    fn new(name: String, desc: String) -> Budget {
        let mut budget = Budget {
            name,
            desc,
            items: HashMap::new(),
        };

        // We use .entry to check if the entry exists.
        // If it does not exist, we insert a value.
        budget.items.entry(String::from("Dummy")).or_insert(500);

        budget
    }
}

fn init_budget() -> Budget {
    let mut name: String = String::new();
    let mut desc: String = String::new();

    println!("Please provide a name for this budget tracker:");
    match stdin().read_line(&mut name) {
        Ok(_) => println!("Thanks."),
        Err(err) => println!("Error: {err}")
    }

    println!("Please provide a description for this budget tracker:");
    match stdin().read_line(&mut desc) {
        Ok(_) => println!("Thanks."),
        Err(err) => println!("Error: {err}")
    }

    Budget::new(name, desc)
}

fn main() {
    let budget = init_budget();

    // This code could be added to a Budget method
    let name = String::from("Dummy");
    let dummy_item: Option<&u32> = budget.items.get(&name);
}
