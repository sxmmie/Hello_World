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
    // let car = String::from("Red");
    // let ref3 = &mut car;
    // ref3.push_str(" and Silver");
    // let ref4 = &car;
    // println!("{ref1} and {ref2} and {}", &car);

    // Ownership with Immutable and Mutable References
    let coffee = String::from("Mocha");
    let a = &coffee;
    let b = a;

    // Dangling references
    // A dangling reference is a pointer to a memory address that has been deallocated
    let city = create_city();
    print!("{city}\n");

    // Ownership with Arrays and Tuples
    // Rust has collection types such as arrays and tuples
    let registrations = [true, false, true];
    let first = registrations[0]; // bool implements the Copy trait
    println!("{first} and {registrations:?}");

    let languages = [String::from("Rust"), String::from("Golang")];
    let first_lang = &languages[0]; // ask for the memory address, therefore not taking ownership of the array from languages
    println!("{first_lang} and {languages:?}");

    // Tuples
    let logins = (true, false, true);
    let first_login = logins.0;
    println!("{first_login} and {logins:?}");
}

// Dangling references
// We are returning a reference to that spot in the heap memory that will no longer be holding the String after the function ends
fn create_city() -> String {
    let city = String::from("New York");
    city
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
