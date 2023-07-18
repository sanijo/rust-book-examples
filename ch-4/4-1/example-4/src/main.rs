// ignore unused variable warning
#![allow(unused_variables)]

fn make_and_drop() {
    let a_box = Box::new(5);
}

fn main() {
    let a_num = 5; // L1 frame in stack contains a_numa // L3 frame in stack contains a_num 
    make_and_drop(); // L2 frame in stack contains a_num and a_box. a_box is dropped
                     // as soon as the function returns
                     
    // ownership transfer (move). When variables frame is deallocated, heap memory
    // of the box is also deallocated.
    let a = Box::new(5); // here a is the owner of the box
    let b = a; // ownership of the box is transferred to b
               // when the scope ends, b is dropped and the box is deallocated
}
