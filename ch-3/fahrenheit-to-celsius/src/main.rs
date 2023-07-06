
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn main() {
    // create array of fahrenheit values
    let fahrenheit_values = [0.0, 32.0, 50.0, 100.0, 212.0];

    // iterate over array of fahrenheit fahrenheit_values and print out
    // the celsius equivalent
    for f in fahrenheit_values.iter() {
        println!("{}°F = {}°C", *f, fahrenheit_to_celsius(*f));
    }
}
