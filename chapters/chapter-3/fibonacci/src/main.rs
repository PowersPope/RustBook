use std::io;

fn main() {

    // Setup our input
    let mut n = String::new();

    // Read in our Nth term
    println!("Input the Nth term to compute in the Fibonacci number (0-index):");
    io::stdin()
        .read_line(&mut n)
        .expect("Could not read line!");
    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    // If we are selecting the first two Nth number then just return them
    if n == 0 { 
        println!("The Nth: {} Fib number is: {}", n, 0);
    } else if n == 1 {
        println!("The Nth: {} Fib number is: {}", n, 1);
    } else {
        // Here is the fibonacci sequence to calculate the nth > 1 value
        let mut _num1: u64 = 0;
        let mut _num2: u64 = 1;

        for _ in 2..n {
            let next_i: u64 = _num1 + _num2;
            _num1 = _num2;
            _num2 = next_i;
        }
        println!("The Nth: {} Fib number is: {}", n-1, _num2);
    }
}

