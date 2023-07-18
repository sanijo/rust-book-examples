fn move_a_box(_b: Box<i32>) {
    // do something
}

fn main() {

    // Below are four snippets which are rejected by the Rust compiler. Imagine 
    // that Rust instead allowed these snippets to compile and run. Select each 
    // snippet that would cause undefined behavior, or select "None of the above" 
    // is none of these snippets would cause undefined behavior.
    
    // snppet 1
    let b = Box::new(0);
    let b2 = b;
    move_a_box(b);

    // snppet 2
    let b = Box::new(0);
    move_a_box(b);
    println!("{}", b);

    // snppet 3
    let b = Box::new(0);
    move_a_box(b);
    let b2 = b;
}

// Explanation snippet 1 and 3:
// Giving b a second owner is undefined behavior, as it would cause Rust to 
// free the box a second time on behalf of b2. It doesn't matter whether the let 
// b2 = b binding happens before or after move_a_box.
//
// Explanation snippet 2:
// Reading b via println after move_a_box is undefined behavior, as it reads 
// freed memory.
