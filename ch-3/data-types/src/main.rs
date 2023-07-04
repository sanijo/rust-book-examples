#![allow(unused_variables)]

use std::io;

fn main() {
    // itneger types
    let number: u32 = "200".parse().expect("Not a number!");
    println!("The number is {}", number);
    let inum: isize = "200".parse().expect("Not a number!");
    println!("The inum value is {}", inum);

    // float types
    // f32, f64 (default)
    let x = 2.2; // f64
    let y: f32 = 3.1; // f32

    // numeric operations
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated_quotient = 5 / 3; // result is 1
    // remainder
    let remainder = 43 % 5;

    // boolean type
    let t = true;
    let f: bool = false; // with explicit type annotation

    // character type is UTF-8 encoded and ranges from U+0000 to U+D7FF and
    // U+E000 to U+10FFFF inclusive
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // compound types
    // tuple type: a general way of grouping together a number of values with 
    // a variety of types into one compound type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // destructuring: to get the individual values out of a tuple directly
    // by using a pattern with a let statement
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    // access a tuple element directly by using a period (.) followed by the    
    // index of the value we want to access
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one: u8 = tup.2; // with explicit type annotation
    // unit tuple: empty tuple with empyty return type    
    let unit_tup = ();

    // array type: every element of an array must have the same type
    // length of an array is part of its type signature and cannot be changed
    // once an array is declared, it is allocated on the stack 
    let a = [1, 2, 3, 4, 5]; // with implicit type annotation
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // with explicit type annotation
    let a = [3; 5]; // same as let a = [3, 3, 3, 3, 3];
    let a: [i32; 5] = [3; 5]; // explicit type annotation
    let a = ["Hello", "World", "!"]; // with implicit type annotation
    // accessing array elements
    let first = a[0];
    let second = a[1];

    // example of accessing array elements by user input of the index
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    // String::new() creates a new empty string instance
    let mut index = String::new(); 
    // read_line() method on the standard input handle to get input from the user
    // &mut index parameter is a reference to the index variable so that read_line
    // can take the user input and place it in that variable
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    println!("Value of array at a[{index}] is: {element}");

    // quiz example 2
    let t = ([1;2], [3;4]); // t = ([1, 1], [3, 3, 3, 3])
    // destructuring
    let (comp1, comp2) = t; // comp1 = [1, 1] and comp2 = [2, 2, 2]
    println!("{}", comp1[0]+t.1[0]);

}
