#![allow(unused_variables)]

// Constant
const TAX_RATE: f64 = 7.25; // you must manually define the type abd value of the value
const TOUCHDOWN_POINTS: i32 = 6;

mod variables;

fn apply_to_jobs(number: i32, title: &str) {
    println!("I'm applying to {number} {title} jobs");
}

fn alphabets(text: &str) -> (bool, bool) {
    (text.contains("a"), text.contains("b"))
    // let a = text.contains("a");
    // a b = text.contains("b");
    // (a, b)
}

fn even_or_odd(number: i32) {
    let result = if number % 2 == 0 { "even" } else { "odd" };
    println!("The number is {result}");
}

// this applies the compiler directory to the main function
fn main() {
    // variables
    variables::run_variables_demo();

    even_or_odd(17);
    even_or_odd(100);

    let jobs = apply_to_jobs(25, "Rust engineer");
    println!("{jobs:?}");

    alphabets("contains");

    // Prints the text in the function
    println!("Hello, world!, learning Rust to be a Rustacean");
    println!("Running my algorithm...");

    /* Multi-line comment */

    // Immutable
    let mut gym_reps = 10; // with the "mut" keywor, we create a binding and make the variable mutable. Though the value can change but the type of the data cannot change
    println!("I plan to to do {gym_reps} reps");
    gym_reps = 15;
    println!("I plan to to do {gym_reps} reps");

    // variable shadowing
    let grams_of_protein = "100.345"; // user input is capturesd as text
    println!("{} is of type string", grams_of_protein);
    // this invalidates the previous var declaration and reassign the value data type
    // remember even when the var is mutable, you can only change the value not the data type, but with this approach you updates the data type and value
    let grams_of_protein = 100.345;
    println!("{} is of type integer", grams_of_protein);

    // scope
    let cookie_price = 3.25;

    {
        let cookie_price = 1.99; // inner scope inside the
        println!("The cookie price is {cookie_price}");
    }

    println!("The cookie price is {cookie_price}");

    // constant
    println!("The tax rate is {TAX_RATE}");

    // type alias - this is nickname or an alternate name that can be assign to an existing type
    type Meters = i32; // creates a type alias Meters which represents an i32 type
    let mile_race_length: Meters = 1600; // metres of a race
    let two_mile_race_length: Meters = 3200;
    println!(
        "A one mile race is {mile_race_length} and a 2 two mile race is {two_mile_race_length} long"
    );

    // compiler directive is an instruction or annotation that tells how to parse the source code.
    #[allow(unused_variables)]
    let mile_race_h: Meters = 1600;

    // Poject
    let season = "Fall";
    let mut points_scored = 32;
    print!("{}", points_scored);
    points_scored = 35;

    let evebt_time = "06:00";
    let event_time = 6;
    print!(
        "My favorite season is {season}.The team scored {points_scored}.mThe event started at {event_time}. A touchdown is worth {TOUCHDOWN_POINTS} points \n"
    );

    let favorite_beverage = "Mountain Dew \n"; // either used the _underscore or the compiler directive to fix the used var

    // Interger
    let eight_bit: i32 = -210;
    let sixteen_bit_signed = 6_230_500;

    let days: usize = 55;
    let years: isize = -15_000;

    // String and Raw String
    let filepath = r"C:\My Documents\new\videos";
    println!("{filepath}");

    // Intro to Methods
    let value: i32 = -15;
    println!("{}", value.abs());

    let empty_space = "     my content      ";
    println!("{}", empty_space.trim());

    // Float Point type
    let pi = 3.14159;
    println!("The current value of pi is {pi}");

    // Formatting Floats with Format Specifier
    // With this, the value of pi is not alter but the output or representation is altered
    let pi = 3.14159453739388984;
    println!("The current value of pi is {pi:.2}"); // this prints out the value of pi but only with 2 digits after the decimal point
    println!("The current value of pi is {:.2}", pi); // this syntax works as well

    // Casting Types - onvert one type to another
    let miles_away = 50; // this remain unchanged
    let miles_away_i8 = miles_away as i8; // casts from one integer type to another integer type

    let miles_away = 100.3290883; // use variable shadowing to redecalre the var to be a floating 64
    let miles_away_f32 = miles_away as f32; // casted from a f64 to f32
    let miles_away_int: i32 = miles_away as i32; // casted from a f32 to i32
    println!("{miles_away_int}");

    // Augmented assignment operators
    let mut year = 2025;
    year = year + 1;
    println!("The new year is {year}");

    // Boolean inversion
    println!("{}", !true); // returns false

    // Character Type . represents a single unicode character
    let first_initial = 'v';
    let emoji = '😂';

    // Array
    let numbers = [4, 8, 15, 15, 23, 42]; // [data type; length of array]
    let apples = ["Granny Smith", "McIntosh", "Red Delicious"];
    println!("Length: {}", apples.len()); // returns the array length

    let currency_rate: [f64; 0] = []; // this is an empty array but the data types must be specified

    // Reading and Writing Array elemtns
    let seasons = ["Spring", "Summer", "Fall", "winter"]; // length counts from 2, index counts from 0
    let first = seasons[0];
    let second = seasons[1];
    println!("The first season is {first}");
    println!("The second season is {second}");

    // Traits - debug trait
    println!("{:?}", seasons);

    // dbg (Debug Macro)
    dbg!(2 + 3);

    // Tuple - stores elements of different types unlike array
    let employee = ("Molly", 32, "Marketing");
    // let name = employee.0;
    // let age = employee.1;
    // let department = employee.2;

    let (name, age, department) = employee; // same as above just shorter
    println!("Name: {name}, age: {age}, department: {department}");

    // Range - represents a sequence/interval or collection of consecutive values
    // Range implements a Debug trait not the Display trait
    let month_days = 1..31; // this range goes up to 31 but does not include 31 (technically its 1-30)
    println!("{month_days:?}");
    let month_days = 1..=31; // this would include the value
    println!("{month_days:?}");

    // itrate over a range
    // for number in month_days {
    //     println!("{number}");
    // }

    // Generics
    let letters = 'b'..'f';
    println!("{letters:?}");
    let generic_letters: std::ops::Range<char> = 'c'..'s';
    println!("{generic_letters:?}");

    // project soln
    let distance = 1337;
    let miles = distance as i16; // cast the i32 to i16

    let height = 150.34546;
    println!("{height:.3}"); // customize how the height value is printed out

    let distances = [13, 23, 75, 100]; // array
    println!("{:#?}", distances);

    let combo = (miles, height, distance, distances); // tupe - a cotainer for different types
    println!("{combo:?}");

    // Function invoked
    open_store("College road");
    cook_afang(2, "goat meat");

    // Return value
    let result = square(5);
    println!("The square of 5 is {result}");

    let result = square(13);
    println!("The square of 13 is {result}");

    let implicit_return_value = square_implicit(7);
    println!("{implicit_return_value}");

    // Unit - is an empty tuple, a tuple without a value
    let result_tuple = (); // empty tuple- this is the defualt return value of a function when a return type is not specified

    // blocks in functions
    let multiplier = 3;
    let calculation = {
        let value = 5 + 4;
        value * multiplier
    };
    println!("{calculation}");

    // Control Flow
    // if statement
    let some_comditions_we_cannot_predict = true;

    if some_comditions_we_cannot_predict {
        println!("This line will be output");
    }

    if false {
        println!("This line will not output anything")
    }

    // match statement
    let evaluation = true;
    match evaluation {
        // pattern/arm
        true => {
            println!("The value is true");
        }
        false => {
            println!("The value is false");
        }
    }

    // another approach
    let value = match evaluation {
        true => 20,
        false => 40,
    };
    println!("This is the {value}");

    // underscore in a match ARM
    let season = "winter";
    // if season == "summer" {
    //     println!("Schools's out");
    // } else if season == "winter" {
    //     println!("Brr, so cold");
    // } else {
    //     println!("Lots of rain");
    // };

    // this is a better refactor (better than the if statement above)
    match season {
        "summer" => println!("Schools's out"),
        "winter" => println!("Brr, so cold"),
        _ => println!("Lots of rain"), // wildcard/catch all pattern
    };

    // match againsta multiple values
    let number = 8; // check if value if even or odd
    match number {
        value if value % 2 == 0 => println!("{value} is an even number"),
        value if value % 2 != 0 => println!("{value} is an odd number"),
        _ => unreachable!(), // this is a macro
                             // _ => println!("Unknown"),
    }

    // the loop and break keyword
    let mut seconds = 10; // keeps track of our seconds remianing
    // let mut seconds = 21;

    // loop {
    //     if seconds == 0 {
    //         println!("Blastoff!");
    //         break; // breaks the loop completely
    //     }

    //     if seconds % 2 == 0 {
    //         println!("{seconds} seconds (even number), skipping 3 seconds..");
    //         seconds -= 3;
    //         continue; // tells the compiler to start from the beginning again
    //     }

    //     println!("{seconds} seconds to blastoff...");
    //     seconds -= 1;
    // }

    // Recursion
    countdown(5);

    // project
    println!("{}", color_to_number("blue"));
    println!("{}", color_to_number("red"));

    println!("{}", factorial(5));

    println!("{}", factorial_recursive(5));

    // Ownership
    let age = 33;
    let is_ugly = false;

    {
        let is_handsome = true; // is_handsome only exists in this scope
    }

    println!("{age}");
    println!("{is_ugly}");
    // println!("{is_handsome}");  // is_handsome is not in this scope

    // Copy Trait - mandate that a type can be copied
    let time = 2026;
    let year = time;
    println!("The time is {time}. It is the year {year}");

    // Ownership and Func Params
    let apples = 6; // this impelemnts the copy Trait
    print_value(apples);
    println!("{apples} is still my value");

    // A String does not implement the copy trait
    let oranges = String::from("Oranges");
    print_string_value(oranges);
    // println!("{oranges} is still my value"); // borrow of moved value - oranges value brrrowed here after move

    // mutable params
    let burger = String::from("Burger"); // string starts out as burger
    add_frries(burger); // let meal = burger    // We transfer the ownership to meal parameter in the add_fries() func

    // Return values I
    let cake = bake_cake();
    println!("I now hvae a {cake} cake");

    // Return values II
    let current_meal = String::new();
    add_flour(current_meal);

    // Project
    // Booleans implements the Copy Trait. For types that implements the Copy Trait, Rust will do a full copy. Like the example below is_event becomes the owner of that copy(duplicate/clone)
    let is_concert = true;
    let is_event = is_concert;
    println!("{is_concert} {is_event}");

    // This type of String that lives on the Heap does not implement the Copy Trait. Therefore a copy is not made and ownership is moved.
    let sushi = String::from("Salmon");
    let dinner = sushi; // dinner is now the owner, can't usesushi after ownership is moved
    // println!("{sushi}"); // sushi value is borrowed here sfter move
    println!("{dinner}");

    // clear()
    eat_meal(dinner); // ownership moves from dinner to meal variable but if we assign it to another variable(fish), then fish becomes the new owner and its accessible in the main func

    // multiple immutable references - A values cna have any number of immutable references
    let car = String::from("Red"); // same data on the heap can be used
    let ref1 = &car;
    let ref2 = &car;
    println!("{ref1} and {ref2} and {}", &car);

    // multiple immutable restrictions - once we declare a single mutable reference, we cannot declare a second refence at the same time
    let mut toyota = String::from("Blue");
    let ref3 = &mut toyota;
    let ref4 = &toyota;
    // ref4.push_str(" and Silver");   // cannot borrow ref4 as a mutable
    println!("{ref4}");

    let mut current_meal = String::new();
    add_flour_to_meal(&mut current_meal);
    show_my_meal(&current_meal);

    // OWnership with immutable and mutable references
    let coffee = String::from("Mocha");
    let a = &coffee; // immutable reference to coffee string
    let b = a; // b referenst s a copy
    println!("{a} and {b}");

    // dangling refernece is a reference to a pointer to a memory address that has been deallocated
    let city = create_city();
    println!("{city}");

    // Ownership with arrays nand tuples
    let registrations = [true, false, true]; // the var registrations is the owner of the array, but the array is the owner of its internal elements
    let first = registrations[0]; // bool implements the copy trait
    println!("{first} and {registrations:?}");

    let languages = [
        String::from("Rust"),
        String::from("Golang"),
        String::from("Python"),
        String::from("Java"),
    ];
    let first_lang = &languages[0]; // heap type on the String does not impelemnt the copy, borrow a ref(ask for the memory addr of the first elem in the array)
    println!("{first_lang} and {languages:?}");

    // tuple
    let registrations_tuple = (true, false, true);
    let first_tuple = registrations_tuple.0;
    println!("{first_tuple} and {registrations_tuple:?}");

    let languages_tuple = (
        String::from("Rust"),
        String::from("Golang"),
        String::from("Python"),
        String::from("Java"),
    );

    let first_lang = &languages_tuple.0;
    println!("{first_lang} and {languages_tuple:?}");

    // Ownership project
    let mut trip = start_trip();
    visit_lagos(&mut trip);
    trip.push_str(" and");
    println!("{trip}");
    visit_new_york(&mut trip);

    // Slices
    // string slice
    let action_hero = String::from("Arnold Schwarzenegger");
    let string_ref = &action_hero;
    println!("{action_hero} and {string_ref}");
}

// Ownership project
fn start_trip() -> String {
    String::from("The plan is...")
}

fn visit_lagos(trip: &mut String) {
    trip.push_str("Lagos State");
}

fn visit_new_york(trip: &mut String) {
    trip.push_str("New York");
}

fn create_city() -> String {
    // dangling refernece is a reference to a pointer to a memory address that has been deallocated
    let city = String::from("Lagos");
    city // return an immutable ref to the string
}

// immutable and mutable reference param
fn add_flour_to_meal(meal: &mut String) {
    meal.push_str("Add flour");
}

fn show_my_meal(meal: &String) {
    println!("Meal steps: {meal}");
}

// Referencing and Borrowing
// fn add_flours() {}

// clear() method mutates the heapr String to have no content.In this example, the meal param is immutable(params declared in a function are immutable by default).
// That means we are not allowed to call or perform any operation that mutates that value and clear() mutates it the value but by putting the mut keyword, we have the permission to mutate the value
fn eat_meal(mut meal: String) -> String {
    meal.clear();
    return meal; // this preserves the String
}

// Return values II
fn add_flour(mut meal: String) {
    meal.push_str("Add flour");
}

// fn add_sugar() {}

// Return values I
fn bake_cake() -> String {
    let cake = String::from("Chocoloate Mousse");
    return cake;
}

// mutable params
fn add_frries(mut meal: String) {
    meal.push_str(" and Fries"); // we mutate the string but concatenating " and Fries" to it
    println!("{meal}");
}

fn print_value(value: i32) {
    println!("Your value is {value}");
}

fn print_string_value(value: String) {
    println!("Your value is {value}");
}

// Project
fn color_to_number(color: &str) -> i32 {
    match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

// Define a factorial function
fn factorial(number: i32) -> i32 {
    let mut product = 1;
    let mut count = number;

    while count > 0 {
        product *= count;
        count -= 1;
    }

    return product;
}

// factorial recursive
fn factorial_recursive(number: i32) -> i32 {
    if number == 1 {
        return 1;
    }

    number * factorial_recursive(number - 1) // 5 * factorial_recursive(4)
}

// Recursive function
fn countdown(seconds: i32) {
    if seconds == 0 {
        println!("Blastoff..");
    } else {
        println!("{seconds} seconds to blastoff..");
        countdown(seconds - 1);
    }
}

// functions
fn open_store(street: &str) {
    println!("Opening my afang store in {street}");
}

fn cook_afang(number: i32, topping: &str) {
    println!("Cooking my {number} afang soup with {topping}");
}

// function return value - every Rust function must have a return value
fn square(number: i32) -> i32 {
    return number * number;
}

// implicit return values - take out the return keywaord and remove the semi-colon in the func and Rust can inferr the the return statement
fn square_implicit(number: i32) -> i32 {
    number * number
}
