fn main() {
    // A struct is a container for related pieces of data.
    // Named Field Structs
    // Tuple-Like Structs
    //  Unit-Like Structs

    // Named Field Structs
    struct Coffee {
        name: String,
        price: f64,
        is_hot: bool,
    }

    // create an instance of the Coffee struct
    let mocha = Coffee {
        name: String::from("mocha"),
        price: 10.95,
        is_hot: true,
    };

    println!("{mocha:?}");
}
