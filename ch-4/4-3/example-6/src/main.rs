// Fixing a safe program: mutating different array elements

// Rust's borrow checker does not contain different paths for a[0], a[1], and 
// so on. It uses a single path a[_] that represents all indexes of a.

fn main() {
//    let mut a = [1, 2, 3];
//    let x = &mut a[0];
//
//    let y = &mut a[1];
//    *x += *y; // Rust will reject this program because a gave its read permission to x

    // This task can be performed with unsafe Rust
    let mut a = [0, 1, 2, 3];
    let x = &mut a[0] as *mut i32;
    let y = &a[1] as *const i32;
    unsafe { *x += *y; } // DO NOT DO THIS unless you know what you're doing!
}

