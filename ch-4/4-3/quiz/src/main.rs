
// q3
// Context: This program is safe. No undefined behavior could occur if it were 
// executed. (If i was outside the bounds of v, then Rust will panic at runtime 
// rather than cause undefined behavior.) The issue is that Rust doesn't know 
// for sure that v[i] and v[i - 1] are referring to different elements.
//fn pass_along(v: &mut Vec<i32>, i: usize) {
//    let n = &mut v[i];
//    *n = v[i-1];
//}

// q4
//fn award_phd(name: &String) {
//    let mut name = *name;
//    name.push_str(", Ph.D.");
//}

fn main() {
    // q2: The string is freed twice at the end of the program.
//    let s = String::from("hello");
//    let s_ref = &s;
////    let s2 = *s_ref; // let s2 = s_ref.clone();
//    let s2 = s_ref.clone();
//    println!("{s2}");

    // q3:
//    let mut v = vec![1, 2, 3];
//    pass_along(&mut v, 1);

    // q4
    // Context: The statement let mut name = *name makes name take ownership of 
    // the input string. However, the caller also still retains ownership of the 
    // string. Therefore after award_phd finishes, the string is deallocated. 
    // Therefore every program below has undefined behavior, because name will 
    // eventually be deallocated a second time. It does not matter whether name 
    // or a reference to name is read after calling award_phd.
//    let name = String::from("Jane");
//    award_phd(&name);
//
//    let name = String::from("Jane");
//    award_phd(&name);
//    println!("{}", name);
//
//    let name = String::from("Jane");
//    let name_ref = &name;
//    award_phd(name_ref);
//    println!("{}", name_ref);

    // q5
    let mut point = [0, 1];
    let mut x = point[0];
    let y = &mut point[1];
    x += 1;
    *y += 1;
    println!("{} {}", point[0], point[1]);
}
