
// Fixing a safe program. Rust can also reject safe programs.
// One common issue is that Rust tries to track permissions at a fine-grained 
// level. However, Rust may conflate two different paths as the same path.

// In this example, Rust will conflate the path name.0 and name.1 as the same path.
// The problem is that Rust doesn't look at the implementation of get_first when 
// deciding what get_first(&name) should borrow. Rust only looks at the type 
// signature, which just says "some String in the input gets borrowed". 
// Rust conservatively decides then that both name.0 and name.1 get borrowed, 
// and eliminates write and own permissions on both.
fn get_first_name(name: &(String, String)) -> &String {
    &name.0
}

fn main() {

    let mut name = (
        "John".to_string(),
        "Doe".to_string()
    );

    let first = get_first_name(&name); // W and O permissions removed from name, name.0 and name.1

    name.1.push_str(" Jr."); // W permission required for name.1
    println!("{first} {}", name.1);
}
