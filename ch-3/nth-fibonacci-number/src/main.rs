
fn nth_fibonnacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        let mut a = 0;
        let mut b = 1;

        for _ in 2..=n { // 2..=n is a range from 2 to n inclusive where _ is a ignored variable
            let tmp = a + b;
            a = b;
            b = tmp;
        }
        b
    }
}

fn main() {

    let n = 9;
    println!("{}th fibonnacci number is {}", n, nth_fibonnacci(n));

}
