#![allow(unused_variables)]

fn some_fn() {
    println!("called `some_fn()`");
}

fn print_integer(x: i32) {
    println!("x is: {x}");
}

fn print_labeled_measurement(value: i32, unit: &str) {
    println!("value is: {value} {unit}");
}

fn return_five() -> i32 {
    5
}

fn int_plus_one(x: i32) -> i32 {
    x + 1
}

fn f(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Main fn");

    some_fn();
    print_integer(5);
    print_labeled_measurement(5, "m");

    // the following is not allowed. This is different from C/C++ where the
    // assignment operator returns the value assigned.
    // let x = let y = 5;

    // the following is allowed. The semicolon is not required.
    let y = {
        let x = 3;
        x + 1 // with semi-colon, this is a statement and not an expression
    };

    println!("y is: {y}");

    let x = return_five();
    println!("return_five() is: {x}");

    let x = int_plus_one(5);
    println!("int_plus_one(5) is: {x}");

    // the following is allowed because code inside {} is an expression that
    // returns a value
    println!("{}", f({let y = 1; y+1}))

}
