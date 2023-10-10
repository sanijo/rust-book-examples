
// tuple structs have the added meaning the struct name provides but don't have
// names associated with their fields; rather, they just have the types of the
// fields. Tuple structs are useful when you want to give the whole tuple a name
// and make the tuple be a different type from other tuples, and naming each
// field as in a regular struct would be verbose or redundant.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// function that takes a Point struct as an argument and destructures it into
// three i32 values by using a let statement
fn print_point(point: Point) {
    let Point(x, y, z) = point;
    println!("x = {}, y = {}, z = {}", x, y, z);
}

// unit-like structs can be useful in situations in which you need to implement
// a trait on some type but don't have any data that you want to store in the
// type itself
struct UnitLikeStruct;

fn main() {
    let mut black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    print_point(origin);

    let unit_like_struct = UnitLikeStruct;

    let x = &mut black.0;
}
