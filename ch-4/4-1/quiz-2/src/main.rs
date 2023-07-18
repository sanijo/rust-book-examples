fn main() {
    let s = String::from("hello");
    let s2;     
    let b = false;  
    if b {
        s2 = s;
    }
    println!("{}", s); // it's illegal to use s here because s could be moved to 
                       // s2 in the if statement
}
