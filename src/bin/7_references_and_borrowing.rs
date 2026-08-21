// Referencing and Borrowing - In rust, a reference is a type of pointer
fn main() {
    let mut current_meal = String::new();
    add_flour(&mut current_meal);
    show_my_meal(&current_meal);

    // Multiple Immutable references
    let car = String::from("Red");
    let ref1 = &car;
    let ref2 = &car;
    println!("{ref1} and {ref2} and {}", &car);

    // Mutable reference Restrictions
    // A value in a program can have any number of immutable reference at the same time
    let car = String::from("Red");
    let ref3 = &mut car;
    ref3.push_str(" and Silver");
    let ref4 = &car;
    println!("{ref1} and {ref2} and {}", &car);
}

// ------ Immutable and Mutable references ----------
// meal: String - I defined a param that would take full ownership of the String and does not have permission to modify it.
// mut meal: String - This means the 'meal' param will take ownership over the String, in addition, it has permission to modify it
// meal: &String - This is no longer a String but a reference to a String. Fundamentally, it's a memory address, but it does not have permission to update the value at the memory
// meal: &mut String (mutable reference to a string) - That means we are not taking ownership of the String. We are receiving a completely different type entirely,
//      a ref and a mutable ref, which means we have permission to go to that memory address, get rhat String and update it however we'd likes

// mutable reference to a string &mut String
fn add_flour(meal: &mut String) {
    meal.push_str("Add flour");
}

// &String means it the address not the text content - meal is now a reference to a string
fn show_my_meal(meal: &String) {
    println!("Meal steps: {meal}")
}
