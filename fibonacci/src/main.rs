use std::io;

fn main() {
    println!("Welcome to Fibonacci!");

    loop {
        println!("Enter an integer to view it's fibonacci value!");

        let mut response = String::new();

        io::stdin()
            .read_line(&mut response)
            .expect("Please enter an integer");

        let response = response.trim();
        let response_int: u128 = match response.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid integer");
                continue;
            }
        };

        let mut fib: u128 = 0;

        if response_int == 0 {
            fib = 0;
        } else if response_int == 1 {
            fib = 1;
        } else {
            let mut prev2 = 0;
            let mut prev1 = 1;

            for _ in 2..response_int + 1 {
                fib = prev1 + prev2;
                prev2 = prev1;
                prev1 = fib;
            }
        }

        println!("The {response} Fibonacci number is {fib}");
    }
}
