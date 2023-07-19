fn get_first(v: &Vec<String>) -> &str {
    &v[0]
}

fn main() {
    let mut strs = vec![
        String::from("A"),
        String::from("B"),
    ];

    let first = get_first(&strs);

    if first.len() > 0 {
        strs.push(String::from("C"));
    }

    let v1 = vec![1, 2, 3];
    let mut v2 = v1;
    v2.push(4);
    println!("{}", v1[0]);

}
