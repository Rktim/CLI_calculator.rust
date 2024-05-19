use std::io;
use std::io::Write;

fn main() {
    loop {
        println!("Enter an expression (e.g., 2 + 3) or 'exit' to quit:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        if input.to_lowercase() == "exit" {
            println!("Exiting calculator...");
            break;
        }

        let result = evaluate_expression(input);
        match result {
            Ok(value) => println!("Result: {}", value),
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

fn evaluate_expression(input: &str) -> Result<f64, String> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() % 2 == 0 {
        return Err("Invalid expression format. Please enter a valid expression.".to_string());
    }

    let mut result = 0.0;
    let mut operator = "+";

    for (index, part) in parts.iter().enumerate() {
        if index % 2 == 0 {
            let num: f64 = match part.parse() {
                Ok(num) => num,
                Err(_) => return Err("Invalid number format.".to_string()),
            };

            match operator {
                "+" => result += num,
                "-" => result -= num,
                "*" => result *= num,
                "/" => {
                    if num == 0.0 {
                        return Err("Division by zero is not allowed.".to_string());
                    }
                    result /= num;
                }
                _ => return Err("Unsupported operator.".to_string()),
            }
        } else {
            operator = *part;
        }
    }

    Ok(result)
}