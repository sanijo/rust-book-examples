#![allow(unused_variables)]

fn main() {
    let number = 7;

    // single if
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if else if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if in a let statement (if is an expression therefore can be used in let)
    let condition = 3 < 5;
    let number = if condition {5} else {6}; // both arms must return the same type
    println!("The condition is {condition} and the number is {number}");

    // quiz example
    // Note that Rust does not require x to be initially declared with let mut 
    // in the second snippet. This is because Rust can determine that x is only 
    // ever assigned once, since only one branch of the if-statement will ever 
    // execute.
    let cond = true;
    let x;
    if cond {
      x = 1;
    } else {
      x = 2;
    }
    println!("The value of x is: {}", x);


    // loops

    // infinite loop with loop keyword
//    loop {
//        println!("again!");
//        break;
//    }

    // return value from loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break returns a value
        }
    };
    println!("The result is {}", result);

    // loop labels to distinguish nested loops
    // Explanation: https://doc.rust-lang.org/book/ch03-05-control-flow.html#loop-labels
    println!();
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // while loop
    println!();
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1; // no -- or ++ in Rust
    }
    println!("LIFTOFF!!!");

    // for loop: iterate over a collection
    println!();
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is {}", element);
    }

    // for loop with range
    println!();
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // quiz
    println!();
    let mut x = 0;
    'a: loop {
        x += 1;
        'b: loop {
            if x > 10 {
                continue 'a;
            } else {
                break 'b;
            }      
        }
        break;       
    }

    println!();

    let a = [5; 10];
    let mut sum = 0;
    for x in a {
        sum += x;
    }
    println!("{sum}");

}
