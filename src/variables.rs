pub fn run_variables_demo() {
    let apples = 50; // i32 is the inferred data type of the variable
    let oranges = 14 + 6;
    let fruits = apples + oranges;
    let _mango = 500; // unused variable

    println!("{}", apples); // interpolation with curly braces "{}"
    println!("{}", oranges);
    println!("Total sum of {} + {} is {}", apples - 10, oranges, fruits);
    // println!("Total sum of {0} + {1} is {2}", apples - 10, oranges, fruits); // this is called positional args
    println!("Total sum of {apples} + {} is {}", oranges, fruits);
}
