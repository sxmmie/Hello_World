fn main() {
    let age = 33;
    println!("{}", age);

    {
        let is_handsome = true;
        println!("{}", is_handsome)
    } // is_handsome goes out of scope here

    // println!("{}", is_handsome); // can't be found in this scope because at this point, the stack entry has been cleared, so the value is gone and the variable is out of scope

    // Copy Trait
    // Integers implements the Copy Trait, Rust will create a full copy of this integer on the stack, the value 2026. This means the 'time' and 'year' variables are independent of each other,
    // they will hold and represent two duplicate, separate, independent copies of the value 2026.
    let time = 2026;
    let year = time;

    println!("The time is {time}. It is the year {year}");

    // String Type
    let food = "pasta"; // this string type is used when we know the exact value at compile time
    // this is stored on the heap, so it can grow and shrink as needed. The new() function allocates an empty string on the heap and returns a pointer to it
    let text_string = String::new(); // this string type is used when we have no idea of the value at compile time. These aredynamic mutable operations, with the :: we are navigating into the String namespace
    let candy = String::from("Kitkat"); // candy is stored on the heap and its the owner of the value. It deallocates when it goes out of scope
    println!("{text_string}");
    println!("{candy}");
    let text_string = text_string + food;
    println!("{text_string}");

    // How to mutate Strings
    // the String text content will live on the heap, but this creation of String will also create a stack entry
    // the stack entry will hold 3 pieces of data - Reference(reference to the String), Length: 5(length of string, which is the current number of bytes the text occupies),
    // Capacity: 10(the amount of bytes available in the heap location). So the Length is "how many byte we are using", the Capacity is "how many bytes we can use".
    // So if more characters are added, the length might expand but the capacity might not, because we do not need to find a new place on the heap, we already have the memory allocated.
    // Variables are immutable by default, so we need to add the immutable modifier "mut"
    let mut name = String::from("Boris");
    println!("{name}");
    println!("Length: {}", name.len()); // returns the number of bytes currently stored in the string (not "characters") returns 5
    println!("Capacity: {}", name.capacity()); // returns how many bytes the string's underlying heap buffer can hold before it needs to reallocate. returns 5

    name.push_str(" Johnson");
    println!("{name}");
    println!("Length: {}", name.len()); // returns 13
    println!("Capacity: {}", name.capacity()); // returns 13

    // If more text content is added to the String and the its more than the current capacity, the String will allocate a new location on the heap and the old location will be deallocated.
    name.push_str(" Andre");
    println!("Length: {}", name.len()); // returns 19
    println!("Capacity: {}", name.capacity()); // returns 26

    // Moves and Ownership
    // Moves - A value can only have one owner at a time but the who that owner is can change(who is responsible for moving that data can change)
    // Copy - A value can have multiple owners but the ownership is not transferable, the value is copied and the ownership is moved to the new owner
    let mut person = String::from("Andre"); // this is string literal(also called a string slice) which is a reference to a String value stored on the heap
    println!("My name is {person}"); // this is valid because the person is till the owner of the String value and the String is stored on the heap
    // Rust copies the previous stack entry, so it copies the reference to the heap data, the length and the capacity from the "person" stack entry and creates a new "genius" stack entry
    // But rust does not copy the text content on the heap. So we have 2 references on the stack but one on the heap. Assigning "person" to "genius", it moves the ownership of the heap data to "genius"
    let genius = person; // genius is assigned the value of person. A heap allocated String does not impelment the Copy Trait, so this is a move not a Copy.
    println!("My name is {genius}");

    // Drop function
    let individual = String::from("Andre");
    drop(individual);

    // Clone
    // we have to tell Rust manually when we do want the duplicate heap data
    let person1 = String::from("Rodri");
    let genius1 = person1.clone();

    println!("This is the {person1}");

    // age variable exists here
} // age variable foes out of scope
