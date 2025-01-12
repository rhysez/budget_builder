pub mod budget {
    use std::collections::HashMap;
    use std::io::stdin;

    pub struct Budget {
        name: String,
        desc: String,
        items: HashMap<String, u32>,
    }

    impl Budget {
        pub fn get_name(&self) -> &String {
            &self.name
        }
    
        pub fn get_desc(&self) -> &String {
            &self.desc
        }
    
        pub fn get_expenses(&self) {
            for (name, cost) in &self.items {
                println!("{name}: {cost}")
            }
        }
    
        pub fn add_expense(&mut self) {
            let mut name: String = String::new();
            let mut cost: String = String::new();
    
            println!("Please provide a name for this expense item:");
            match stdin().read_line(&mut name) {
                Ok(_) => println!("Thanks."),
                Err(err) => println!("Error: {err}"),
            }
        
            println!("Please provide an associated cost for this expense item:");
            match stdin().read_line(&mut cost) {
                Ok(_) => println!("Thanks."),
                Err(err) => println!("Error: {err}"),
            }
    
            // We shadow (move) cost and parse to a u32. We must handle the potential error.
            // An error may be thrown if a user does not enter a valid number as a character.
            let cost: Result<u32, _> = cost.trim().parse();
    
            // If cost is u32, we insert it into the hashmap.
            // Else, the program panics. 
            match cost {
                Ok(number) => {
                    self.items.entry(name).or_insert(number);
                },
                Err(error) => panic!("Error: {error}"),
            }
    
            println!("Displaying up-to-date list of expenses...");
            self.get_expenses()
        }
    }
    
    impl Budget {
        pub fn new(name: String, desc: String) -> Budget {
            Budget {
                name,
                desc,
                items: HashMap::new(),
            }
        }
    }
}