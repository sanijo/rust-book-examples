fn incr(n: &mut i32) {
    *n += 1;
}

fn main() {
    let mut n = 1;
//    incr(& n); // wont compile
    incr(&mut n);
    println!("{n}");

    let mut s = String::from("hello");
    let s2 = &s;
    let s3 = &mut s;
    s3.push_str(" world");
//    println!("{s2}"); // wont compile
}
