use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
//    let x = 69; // variables are immutable by default
    let mut x = 69;
    println!("The value of x is: {x}");
    x = 420;
    println!("The value of x is: {x}");

    // constants are always immutable, can be declared in any scope, and can 
    // be set only to a constant expression
    const MAX_POINTS: u32 = 100;
    println!("The value of MAX_POINTS is: {MAX_POINTS}");


    // shadowing allows us to change the type of the variable while keeping the 
    // same 
    println!();
    let x = x + 1;
    println!("The value of x is: {x}");
    {
        let x = x * 2;
        println!("The value of x is: {x}");
    }
    println!("The value of x is: {x}");

    println!();
    let spaces = "    ";
    // print the type and value of spaces
    println!("The type of spaces is: {}", type_of(spaces));
    let spaces = spaces.len();
    println!("The type of spaces is: {}", type_of(spaces));
}
