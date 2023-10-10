// Problem: trying to mutate read only data or trying to drop data
// behind a reference.

// This wont compile because we are trying to mutate a read only data
//fn stringify_name_with_title(name: &Vec<String>) -> String {
//    name.push(String::from("Esq."));
//    let full = name.join(" ");
//    full
//}

// Solution 1: change type of name to mutable (not recommended if the caller 
// would not expect the data to be mutated by the function)
//fn stringify_name_with_title(name: &mut Vec<String>) -> String {
//    name.push(String::from("Esq."));
//    let full = name.join(" ");
//    full
//}

// Solution 2: take ownership of the input (not recommended because it would make
// the input unusable after the function call)
//fn stringify_name_with_title(mut name: Vec<String>) -> String {
//    name.push(String::from("Esq."));
//    let full = name.join(" ");
//    full
//}

// Solution 3: clone the input 
//fn stringify_name_with_title(name: &Vec::<String>) -> String {
//    let mut name_clone = name.clone();
//    name_clone.push(String::from("Esq."));
//    let full = name_clone.join(" ");
//    full
//}

// Solution 4: avoid mutating the input by pushing to the joined string rather
// than the vector
fn stringify_name_with_title(name: &Vec::<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

fn main() {
    let name = vec![String::from("John"), String::from("Doe")];
    let first = &name[0];
//    stringify_name_with_title(&name);
    let _stringified = stringify_name_with_title(&name);
    println!("{first}");
}
