use std::io::{self, Write};
#[derive(Debug)]

struct Operation{
    left: i32,
    right: i32,
    operator: char,
}

fn create_operation()-> Operation{
    Operation{left: 0, right: 0, operator: '+'}
}

fn main() {
    println!("Welcome to RustyBeer!");
    let mut input = String::new();

    while input == "" {
        let mut operation = create_operation();

        println!("Enter first input:");
        io::stdin().read_line(&mut input).expect("Failed to read the line");
        operation.left = input.trim().parse::<i32>().expect("That's not a number");
        io::stdout().flush().unwrap();
        input.clear();

        println!("Enter operator: ");
        io::stdin().read_line(&mut input).expect("Failed to read the line");
        operation.operator = input.trim().parse::<char>().expect("That's not a character");
        io::stdout().flush().unwrap();
        input.clear();

        println!("Enter second input: ");
        io::stdin().read_line(&mut input).expect("Failed to read the line");
        operation.right = input.trim().parse::<i32>().expect("That's not a number");
        input.clear();

        match operation.operator {
            '+' => println!("The answer is: {}", operation.left + operation.right),
            '-' => println!("The answer is: {}", operation.left - operation.right),
            '*' => println!("The answer is: {}", operation.left * operation.right),
            '/' => println!("The answer is: {}", operation.left / operation.right),
            _ => println!("Invalid operator")
        };

        println!("Would you like another calculation? (y/n)");
        io::stdin().read_line(&mut input).expect("Answer not applicable");
        if input.trim().parse::<char>().expect("That's not a character") == 'y'{
            input.clear();
        }
        else { input = "q".to_string(); }
    }
}
