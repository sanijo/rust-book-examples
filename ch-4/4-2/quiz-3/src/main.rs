
fn give_and_take(v: &Vec<i32>, n: i32) -> i32 {
    v.push(n);
    v.remove(0)
}

fn main() {
    let v = vec![1, 2, 3];
    let n = &v[0];
    give_and_take(&v, 4);
    println!("{}", n);
}

// Description:
// v.push(n) can cause v to reallocate its internal contents, invalidating any 
// references to the elements of v on the heap. Therefore calling give_and_take(&v, 4) 
// will cause previously-created element references to point to invalid memory.
