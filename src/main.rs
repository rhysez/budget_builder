use std::collections::HashMap;

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

    fn get_all_items(&self) {
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

fn main() {
    let mut budget: Budget = Budget::new(String::from("Test"), String::from("Example description"));

    // This code could be added to a Budget method
    let name = String::from("Dummy");
    let dummy_item: Option<&u32> = budget.items.get(&name);
    match dummy_item {
        Some(dummy_item) => println!("{}: {}", &name, &dummy_item),
        None => println!("Item does not exist under name of {}.", &name),
    }
}
