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
}
