// returning a refference to the stack

// the issue is with the lifetime of the referred data: the data is on the stack
// and will be deallocated when the function returns, so the reference will be 
// invalid
//fn return_string() -> &String {
//    let s = String::from("hello");
//    &s
//}

// Solution 1: return the String itself (move ownership)
//fn return_string() -> String {
//    let s = String::from("hello");
//    s
//}

// Solution 2: return string literal (live for the entire program which is noted
// by the 'static lifetime)
//fn return_string() -> &'static str {
//    "hello"
//}

// Solution 3: use reference counting
use std::rc::Rc;

fn return_string() -> Rc<String> {
    let s = Rc::new(String::from("hello"));
    Rc::clone(&s)
}

// Solution 4: provide "slot" to put string using a mutable reference
//fn return_string(output: &mut String) {
//    output.replace_range(.., "hello");
//}

fn main() {
    let s = return_string();
    println!("{s}");

//    let mut s = String::new();
//    return_string(&mut s);
//    println!("{}", s);
}
