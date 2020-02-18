use std::io;
use std::process;
    
fn main() {
    println!("First value: ");
    let mut sum = String::new();
    
    io::stdin()
        .read_line(&mut sum)
        .expect("Numeric value expected");

    let mut sum: f64 = sum
        .trim()
        .parse()
        .unwrap_or_else(|e| {
            println!("Error: {}", e);
            process::exit(1);
        });
    
        loop {
            let mut operation: String = String::new();
            println!("[+]");
            println!("[-]");
            println!("[*]");
            println!("[/]");
            println!("[%]");

            io::stdin().read_line(&mut operation).unwrap_or_else(|e| {
                println!("Error: {}", e);
                process::exit(1);
            });
            
            let operation = operation.trim() as &str;
            match operation{
                "+" => {
                    println!("Type number to add to the current value {}", sum);
                    let operation = convert();
                    println!("{} + {} = {}", sum, operation, sum + operation);
                    sum += operation;
                },
                "-" => {
                    println!("Type number to substract to the current value {}", sum);
                    let operation = convert();
                    println!("{} - {} = {}", sum, operation, sum - operation);
                    sum -= operation;
                },
                "*" => {
                    println!("Type number to substract to the current value {}", sum);
                    let operation = convert();
                    println!("{} * {} = {}", sum, operation, sum * operation);
                    sum *= operation;
                },
                "/" => {
                    println!("Type number to substract to the current value {}", sum);
                    let operation = convert();
                    println!("{} / {} = {}", sum, operation, sum / operation);
                    sum /= operation;
                },
                "%" => {
                    println!("Type number to substract to the current value {}", sum);
                    let operation = convert();
                    println!("{} % {} = {}", sum, operation, sum % operation);
                    sum %= operation;
                },
                _ => {
                    println!("Invalid operation.");
                    process::exit(1);
                },
            }
        }
}

fn convert() -> f64 {
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)
        .unwrap_or_else(|_| { println!(); process::exit(1); });
    let choice: f64 = choice.trim()
        .parse()
        .unwrap_or_else(|_| { println!(); process::exit(1); });
        
    choice
}