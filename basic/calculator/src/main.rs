use std::io;
fn main() {
    println!(r#"Provide two numbers and an operator separated by spaces
Available numbers: ints and floats separated with period (.)
Available operators: + - * / % ^
Example: 45 35.6 +
"#);

    let mut todo_calculus = String::new();

    io::stdin()
        .read_line(&mut todo_calculus)
        .expect("Failed to read line");

    let parts: Vec<&str> = todo_calculus.split(' ').collect();

    let numbers_array: [f64; 2] = parse_numbers(parts);

    println!("{:?}", numbers_array);
}


fn parse_numbers(vector: Vec<&str>) -> [f64; 2]  {
    let mut numbers_array: [f64; 2] = [0.0; 2];
    for i in 0..2 {
        if let Ok(number) = vector[i].parse::<f64>() {
            numbers_array[i] = number;
        } else {
            println!("Couln't parse the {} number", i)
        }
    }
    
    numbers_array
}