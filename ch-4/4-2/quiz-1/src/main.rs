fn get_first(v: &Vec<String>) -> &String {
    &v[0]
}

fn get_first_mut(v: &mut Vec<String>) -> &mut String {
    &mut v[0]
}

fn main() {
    // example 1: using immutable reference
    let mut strs = vec![
        String::from("A"),
        String::from("B"),
    ];

    let first = get_first(&strs);

    if first.len() > 0 {
        strs.push(String::from("C"));
    }

//    println!("{}", first); // this won't compile 

    // example 2: using mutable reference
    let mut strs = vec![
        String::from("A"),
        String::from("B"),
    ];

    let first = get_first_mut(&mut strs);

    // replace string "A" with "D"
    first.remove(0);
    first.insert(0, 'D');

    if first.len() > 0 {
        strs.push(String::from("C"));
    }

    println!("{:?}", strs); // this will compile


//    let v1 = vec![1, 2, 3];
//    let mut v2 = v1;
//    v2.push(4);
//    println!("{}", v1[0]);

}
