fn main() {
    let first = String::from("John"); 
    // cloning can help to avoid moving heap data
    // let first_clone = first.clone();
    let full = add_suffix(first);
    // let full = add_suffix(first_clone);
    println!("Hello, {full}!");
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

// Move heap data principle: if a variable x moves ownership of heap data to 
// another variable y, then x can no longer be used.
