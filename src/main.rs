use std::io;
use std::io::prelude::*;

// Basic calculator that reads input from the user and performs operations
fn main() {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let input = line.unwrap();

        if input == "exit" {
            break;
        }

        // Split the input into parts
        let parts: Vec<&str> = input.split_whitespace().collect();

        // Ensure there are exactly 3 parts: number, operator, number
        if parts.len() != 3 {
            println!("Please enter an operation in the format: number operator number");
            continue;
        }

        // Parse the numbers
        let num1: f64 = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number: {}", parts[0]);
                continue;
            }
        };
        let num2: f64 = match parts[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid number: {}", parts[2]);
                continue;
            }
        };

        // Perform the operation
        let result = match parts[1] {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => num1 / num2,
            _ => {
                println!("Invalid operator: {}", parts[1]);
                continue;
            }
        };

        // Print the result
        println!("{} {} {} = {:.2}", num1, parts[1], num2, result);
    }
}