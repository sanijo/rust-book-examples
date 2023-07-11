#![allow(unused_variables)]

fn main() {
    let a = Box::new([0,1,2,3,4,5]);
    let mut b = a;
    b[0] = 6;

    // print b[0]
    println!("{}", b[0]);
}
