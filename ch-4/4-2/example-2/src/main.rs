// allow unused variable
#![allow(unused_variables)]

fn main() {
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x; // a = 1, reads heap value through dereferenced pointer
    *x += 1; // x = 2, increments heap value through dereferenced pointer

    let r1: &Box<i32> = &x; // r1 = &2, creates a reference to x on the stack
    let b: i32 = **r1; // b = 2, reads heap value through double dereferenced pointer

    let r2: &i32 = &*x; // r2 = &2, points to the heap value through dereferenced pointer
    let c: i32 = *r2; // c = 2, so only one dereference is needed to read the heap value

    // examples of imlicit and explicit dereferencing
    let x: Box<i32> = Box::new(-1);
    let x_abs_e = i32::abs(*x); // explicit dereferencing
    let x_abs_i = x.abs(); // implicit dereferencing
    assert_eq!(x_abs_e, x_abs_i);

    let r1: &Box<i32> = &x;
    let r1_abs_e = i32::abs(**r1); // explicit dereferencing
    let r1_abs_i = r1.abs(); // implicit dereferencing
    assert_eq!(r1_abs_e, r1_abs_i);

    let r2: &i32 = &*x;
    let r2_abs_e = i32::abs(*r2); // explicit dereferencing
    let r2_abs_i = r2.abs(); // implicit dereferencing
    assert_eq!(r2_abs_e, r2_abs_i);

    let s = String::from("hello");
    let s_len1 = str::len(&s); // explicit dereferencing
    let s_len2 = s.len(); // implicit dereferencing

    let x = Box::new(-1);
    let y = Box::new(&x);
    // get value of x through y
    let y_x_e = ***y; // explicit dereferencing
    println!("y_x_e = {}", y_x_e);
}
