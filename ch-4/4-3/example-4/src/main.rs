// Copying vs. moving out of a collection

fn main() {
// This will compile
//    let v = vec![1, 2, 3];
//    let n_ref = &v[0];
//    let n = *n_ref;

// This will not compile because both s and v expect to own the string which 
// leads to a double free error and undefined behavior.
// Explanation: this undefined behavior does not happen when the vector contains 
// i32 elements. The difference is that copying a String copies a pointer to heap 
// data. Copying an i32 does not. In short, if a value does not own heap data,
// then it can be copied without move.
//    let v = vec!["hello".to_string()];
//    let s_ref = &v[0];
//    let s = *s_ref;

// Solution 1: avoid taking ownership of the vector and just use immutable references
//    let v = vec!["hello".to_string()];
//    let s_ref = &v[0];
//    println!("s_ref = {s_ref}");

// Solution 2: clone the string data
//    let v = vec!["hello".to_string()];
//    let mut s = v[0].clone();
//    s.push_str(", world!");
//    println!("s = {s}");

// Solution 3: use method like Vec::remove() that moves the String out of the 
// vector and take ownership of it
    let mut v = vec!["hello".to_string()];
    let mut s = v.remove(0);
    s.push_str(", world!");
    println!("s = {s}");
    assert!(v.is_empty());
}
