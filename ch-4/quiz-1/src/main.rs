fn add_suffix(mut s: String) -> String {
    s.push_str(", world");
    s
}

fn main() {
    let s = String::from("hello");
    let s2 = add_suffix(s);
    println!("{}", s2);
}

// 1. s holds pointer to "hello" string in heap
// 2. ownership of s is moved to add_suffix function, specifically to s parameter
// 3. push_str method appends ", world" to the string in newly allocated memory
// 4. ownership is moved back to s2 when add_suffix function returns
