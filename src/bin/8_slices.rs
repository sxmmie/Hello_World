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
}

fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day");
}
