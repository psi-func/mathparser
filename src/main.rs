use std::io;
use mathparser::evaluate;

fn main() {
    println!(
        "Hello! Welcome to Arithmetic expression
evaluator."
    );
    println!(
        "You can calculate value for expression such as
2*3+(4-5)+2^3/4. "
    );
    println!(
        "Allowed numbers: positive, negative and
decimals."
    );
    println!(
        "Supported operations: Add, Subtract, Multiply,
Divide, PowerOf(^). "
    );
    println!("Enter your arithmetic expression below:");
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match evaluate(input) {
                    Ok(val) => println!(
                        "The computed number
is {}\n",
                        val
                    ),
                    Err(_) => {
                        println!(
                            "Error in evaluating
expression. Please enter valid
expression\n"
                        );
                    }
                };
            }
            Err(error) => println!("error: {}", error),
        }
    }
}
