fn main() {
    // Box is used to allocate memory on the heap
    let a = Box::new([0; 10]);
    // following statement copies the pointer from a to b
    let b = a;

    // print elements of b array
    for el in b.iter() {
        println!("{}", el);
    }
}
