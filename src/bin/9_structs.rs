#[derive(Debug)]
// Named Field Structs
struct Coffee {
    name: String,
    price: f64,
    is_hot: bool,
}

#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

// methods of TaylorSwiftSong struct
impl TaylorSwiftSong {
    // immutable struct value (self parameter takes ownership)
    fn display_song_info(self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
    }

    // Mutable struct value (self parameter takes ownership, has permission to mutate)
    // sometimes we want to update some (not all) fields in a struct
    fn double_lenth(mut self) {
        self.duration_secs = self.duration_secs * 2;
        println!("{:#?}", self)
    }

    // Immutable reference to the struct instance (no ownership moved)
    // Mutable reference to the struct instance (no ownership moved, have permission to mutate)
}

// Self Param as immutable and mutable references to struct instances

fn main() {
    // Defining Struct Methods
    let song = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2015,
        duration_secs: 231,
    };

    song.display_song_info();

    // song.double_lenth();

    // A struct is a container for related pieces of data.
    // Named Field Structs
    // Tuple-Like Structs
    // Unit-Like Structs

    // create an instance of the Coffee struct
    let mut beverage = Coffee {
        name: String::from("Mocha"),
        price: 10.95,
        is_hot: true,
    };

    // Accessing the values of the struct fields
    println!("My {} this morning cost {}. It is {} that it was hot", beverage.name, beverage.price, beverage.is_hot);

    // Overwrite the Struct fields - add the "mut" keyword in the instance of the struct - All fields must be mutable
    beverage.name = String::from("Caramel Macchiato");
    beverage.price = 6.99;
    beverage.is_hot = false;
    println!("My {} this morning cost {}. It is {} that is was hot", beverage.name, beverage.price, beverage.is_hot);

    // Create struct in a Function
    let name = String::from("Latte");
    let coffee = make_coffee(name, 4.99, true);
    println!("My {} this morning cost {}. It is {} that it was hot", coffee.name, coffee.price, coffee.is_hot);

    // Struct Update Syntax
    let caramel_macchiato = Coffee {
        // name: String::from("Caramel Macchiato"),
        name: coffee.name.clone(), // clone creates a duplicate of the data on the heap
        ..coffee
    };

    println!("{}", caramel_macchiato.name);

    // Passing a Struct in a function as an argument
    let mut mocha = make_coffee(String::from("mocha"), 4.99, true);
    drink_coffee(&mut mocha);

    println!("{}, {}", mocha.name, mocha.price);

    // Self Param as Mutable struct instance
}

// Passing a Struct in a function as an argument
fn drink_coffee(coffee: &mut Coffee) {
    println!("Drinking my delicious {}", coffee.name);
    coffee.is_hot = false;
    coffee.price = 5.99;
}

// Create struct in a Function
fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee {
    Coffee {
        name: name,
        price: price,
        is_hot: is_hot,
    }
}
