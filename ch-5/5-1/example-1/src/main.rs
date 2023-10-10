// ignore unused variable warning
#![allow(unused_variables)]
// ignore unused field warning
#![allow(dead_code)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Function that returns new user instance
fn new_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("john.doe@gmail.com"),
        sign_in_count: 1,
    };

    user1.active = false;

    // create new user instance using function  
    let user2 = new_user(String::from("new@user.com"), String::from("newuser"));

    // create new user instance using struct update syntax
    // ..user1 means use all the fields of user1 for the remaining fields which 
    // are not explicitly set
    let user3 = User {
        username: String::from("user3"),
        email: String::from("a@b.com"),
        ..user1
    };

    user1.username = String::from("user1_updated");

    // print user1 email
    println!("user1 email: {}", user1.email);
}
