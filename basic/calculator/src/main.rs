use std::io;

fn main() {
    println!(r#"Provide two numbers and an operator separated by spaces
Available numbers: ints and floats separated with period (.)
Available operators: + - * / %
Example: 45 35.6 +
"#);

    let mut todo_calculus = String::new();

    let mut condition: bool = true;
    let mut numbers_array: Option<[f64; 2]> = None;
    let mut operation: Option<char> = None;

    while condition {
        io::stdin()
        .read_line(&mut todo_calculus)
        .expect("Failed to read line");
        let parts: Vec<&str> = todo_calculus.split(' ').collect();
        numbers_array = parse_numbers(&parts);
        operation = parse_operation(&parts);

        if numbers_array == None || operation == None {
            if numbers_array == None {
                println!("One of the numbers provided is not a correct number");
            }          
            if operation == None {
                println!("Not a valid operation");
            }

            continue
        }

        condition = false;

    }

    let result = make_operation(numbers_array, operation);

    println!("{}", result);

}

fn parse_operation(vector: &Vec<&str>) -> Option<char> {
    let character = vector[2].chars()
                                   .next()
                                   .expect("Expected a value, but found None");
    let addmisible_operators = ['+', '-', '*', '/', '%'];
    if addmisible_operators.contains(&character) {
        Some(character)
    } else {
        None
    }
}


fn parse_numbers(vector: &Vec<&str>) -> Option<[f64; 2]>  {
    let mut numbers_array: [f64; 2] = [0.0; 2];
    for i in 0..2 {
        if let Ok(number) = vector[i].parse::<f64>() {
            numbers_array[i] = number;
        } else {
            return None
        }
    }
    
    Some(numbers_array)
}

fn make_operation(numbers_array: Option<[f64; 2]>, operation:  Option<char>) -> f64{
    let numbers = numbers_array.unwrap();
    let operation = operation.unwrap();

    let result = match operation {
        '+' => numbers[0] + numbers[1],
        '-' => numbers[0] - numbers[1],
        '*' => numbers[0] * numbers[1],
        '/' => numbers[0] / numbers[1],
        '%' => numbers[0] % numbers[1],
        _ => panic!("Unexpected operation error"),
    };
    

    result
}