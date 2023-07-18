fn main() {
    let first = String::from("John"); 
    // cloning can help to avoid moving heap data
    // let first_clone = first.clone();
    let full = add_suffix(first);
    // let full = add_suffix(first_clone);
    println!("full name is {full}");
//    println!("{full}, originally {first}!"); // undefined behavior: pointer used after its pointee is freed
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

// Move heap data principle: if a variable x moves ownership of heap data to 
// another variable y, then x can no longer be used.

// Program steps:
// 1. main() creates a String object on the heap and binds it to first.
// 2. main() calls add_suffix() and passes first as an argument. ownership of 
// the string is moved from first to name.
// name.push_str() resizes the string on the heap. Larger allocation is createsd 
// first, then "John Jr." is written in new location. The old allocation is 
// deallocated, first now points to freed memory.
// 3. add_suffix() returns the string to main(). Ownership of the string is 
// transferred from name to full.
