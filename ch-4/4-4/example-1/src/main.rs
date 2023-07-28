// Slices let you reference a contiguous sequence of elements in a collection 
// rather than the whole collection. A slice is a kind of reference, so it is 
// a non-owning pointer.

// Slices have the type &[T], where T is the type of the elements. For example,
// a slice of i32 values would have the type &[i32].

// Write a function that takes a string of words separated by spaces and returns 
// the first word it finds in that string.

fn first_word(s: &String) -> usize {
    // convert string to bytes slice
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() { // iter() returns each element in a collection
        if item == b' ' { // b' ' is a byte literal
            return i;
        }
    }
    s.len()
}

// if &str is used in function signature instead of &String, the function can 
// accept both &String and &str. This is called deref coercion.
fn first_word_string_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() { // iter() returns each element in a collection
        if item == b' ' { // b' ' is a byte literal
            return &s[..i]; // return a string slice from index 0 up to the index i
        }
    }

    &s[..] // return the whole string
}

fn main() {
//    let mut s = String::from("hello world");
//    let word = first_word(&s); 
//    s.clear();

    let s = String::from("hello world");
    let hello = &s[0..5]; // &s[..5] is the same
    let world = &s[6..]; // &s[6..11] is the same
    let s2 = &s; //reference to the whole string
    let helloworld = &s[..]; // &s[0..11] is the same
                 
    println!("{} {} {} {}", hello, world, s2, helloworld);

    let mut s = String::from("Some words are here");
    let word = first_word_string_slice(&s[..]);
    println!("the first word is: {}", word);
    s.clear();
//    println!("the first word is: {}", word); // this will cause error

    let s = String::from("And some words are here"); 
    let word = first_word_string_slice(&s[4..]);
    println!("the first word is: {}", word);

    let s = "Same situation here"; // string literal is a slice
    let word = first_word_string_slice(s);
    println!("the first word is: {}", word);

    // Example of array slices 
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // slice has the type &[i32]
    println!("slice: {slice:?}");

//    let mut s = String::from("hello");
//    for &item in s.as_bytes().iter() { // as_bytes() returns immutable reference to a byte slice
//        if item == b'l' {
//            s.push_str(" world");
//        }
//    }
}
