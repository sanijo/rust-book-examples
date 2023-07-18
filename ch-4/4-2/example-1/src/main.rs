
// Commented code won't work because greet function takes ownership of m1 and m2
// To avoid this, we can pass references to m1 and m2 to greet function. 
// This way, greet function will not take ownership of m1 and m2.
// References are non-owning pointers. They point to a value but don't own it.
// They are immutable by default. To make them mutable, we need to use mut keyword.

fn main() {
    let m1 = String::from("Hello");
    let m2 = String::from("World");

//    greet(m1, m2);
//    let s = format!("{} {}!", m1, m2); // won't work because m1 and m2 are 
                                         // moved to greet function

    greet(&m1, &m2);
    let s = format!("{} {}!", m1, m2);
    println!("{}", s);
}

//fn greet(g1: String, g2: String) {
//    println!("{}, {}!", g1, g2);
//}

fn greet(g1: &String, g2: &String) {
    println!("{}, {}!", g1, g2);
}
