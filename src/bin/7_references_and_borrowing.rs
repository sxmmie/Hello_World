// Referencing and Borrowing - In rust, a reference is a type of pointer
fn main() {
    let mut current_meal = String::new();
    add_flour(&mut current_meal);
    show_my_meal(&current_meal);
}

// meal: String
// mut meal: String

// mutable reference to a string &mut String
fn add_flour(meal: &mut String) {
    meal.push_str("Add flour");
}

// &String means it the address not the text content - meal is now a reference to a string
fn show_my_meal(meal: &String) {
    println!("Meal steps: {meal}")
}
