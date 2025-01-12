use rusty_budget::utils::init_budget;

fn main() {
    let mut budget = init_budget();
    budget.add_expense();
}
