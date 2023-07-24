
// Another unsafe operation is using a reference to heap data that gets 
// deallocated by another alias.

// For example, here's a function that gets a reference to the largest string 
// in a vector, and then uses it while mutating the vector:

// Here largest removes W permission from dst. However dst.push requires W.
// Solution is to shorten the lifetime of largest.
//fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap(); // removes W permission
//                                                                        // from dst
//
//    for s in src {
//        if s.len() > largest.len() {
//            dst.push(s.clone());
//        }
//    }
//}

// Solution 1 is to clone largest:
// Problem: this is inefficient due to the clone
//fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone(); 
//
//    for s in src {
//        if s.len() > largest.len() {
//            dst.push(s.clone());
//        }
//    }
//}

// Solution 2: perform all the length comparisons first, and then mutate dst afterwards
// Problem: also causes performance hit for allocating the vector to_add
//fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
//
//    let to_add: Vec<String> = src.iter()
//        .filter(|s| s.len() > largest.len())
//        .cloned()
//        .collect();
//
//    dst.extend(to_add);
//}

// Solution 3: copy length of largest into a variable, and then use that variable
// since largest is not necessary anymore
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len = dst.iter().max_by_key(|s| s.len()).unwrap().len();

    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

fn main() {
    let mut v = vec!["hello".to_string(), "world".to_string()];
    let src = vec!["foooooooj".to_string(), "bar".to_string(), "baz".to_string()];

    add_big_strings(&mut v, &src);

    println!("{:?}", v);
}
