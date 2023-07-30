fn mut_num(x: &mut i32) {
    let another_num = 1;
    let _a_stack_ref = &another_num;

    let a_box = Box::new(2);
    let _a_box_stack_ref = &a_box;
    let _a_box_heap_ref = &*a_box;

    *x = 5;
}

fn main() {
    let mut num = 1;
    mut_num(&mut num);
    println!("num is {num}");

    let mut v = vec![1, 2, 3, 4, 5];
    let (a, b) = v.split_at_mut(2);
    a[0] = 10;
    b[0] = 20;
}
