// A slice is a reference to a portion/sequence of a collection type. It’s a subcategory og reference.
fn main() {
    // String slice
    let action_hero = String::from("Arnold Schwarzenegger"); // A string slice from a heap-allocated string
    let first_name = &action_hero[0..6];
    println!("{first_name}");

    let last_name = &action_hero[7..21];
    println!("{last_name}");

    // String Slices and String Literal
    let first_name_hero = {
        let string_lit = "Arnold Schwarzenegger";
        &string_lit[0..6]
    };

    println!("{first_name_hero}");

    // String Slice Length
    // The length of a string slice refers to a count of it bytes, not its characters
    let food = "🍕";
    println!("{}", food.len());

    // let pizza_slice = &food[0..3];
    // println!("{}", pizza_slice.len())

    let my_name = "Samuel Umoh";
    let full_name = &my_name[..];
    println!("{full_name}");

    // String Slices as Function Parameters
    let my_hero_name = String::from("R9");
    do_hero_stuff(&my_hero_name);
    let another_hero_name = "CR7";
    do_hero_stuff(&another_hero_name); // passing a String not a &str slice

    // Array Slices
    let values = [4, 8, 15, 16, 23, 42];
    let my_slice = &values[0..3];
    println!("{values:?} {my_slice:?}");
    let my_slice = &values[2..4];
    println!("{my_slice:?}");

    // Deref Coercion with Array Slices
    let regular_reference = &values;
    print_length(regular_reference);

    let slice_of_three = &values[..3];
    print_length(slice_of_three);

    // Mutable Array Slices
    let mut my_array = [10, 15, 20, 25, 30];
    let my_num_slice = &mut my_array[2..4]; // a mutable ref to a an array(a portion of the whole)
    println!("My slice {:?}", my_num_slice);

    my_num_slice[0] = 100; // modifies the original values of the array
    println!("My slice {:?}", my_num_slice);
    print!("My array {:?}\n", my_array);

    // Projects - Solution
    let mut cereals = [
        String::from("Cookie Crisp"),
        String::from("Cinnamon Toast Crunch"),
        String::from("Frosted flakes"),
        String::from("Cocoa Puffs"),
        String::from("Captain Crunch"),
    ];

    let first_two = &cereals[..2];
    print!("The first two cereals {:?}\n", first_two);

    let mid_three = &cereals[1..4];
    print!("The mid three creals {:?}\n", mid_three);

    let last_three = &mut cereals[2..];
    println!("The last three cereals {:?} \n", last_three);

    // replace the last element(Captain Crunch) in the last_three slice with "Luckey charms". Print the complete cereal array
    last_three[2] = String::from("Lucky Charms");
    println!("{last_three:?}");
    println!("{cereals:?}");

    // Declare cookie_crisp variable. Make it a reference to the Cookie Crisp String (in other words, a &String)
    let cookie_crisp = &cereals[0];
    let cookie = &cookie_crisp[0..6];
    println!("{cookie}");
}

// Deref Coercion with Array Slices
fn print_length(reference: &[i32]) {
    println!("{}", reference.len());
}

fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day");
}
