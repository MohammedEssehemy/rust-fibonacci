use std::{
    io,
    time::Instant
};
use fibonacci_series::{fibonacci_non_recursive, fibonacci_recursive};

fn main() {
    println!("Fibonacci Series");

    loop {
        println!("please input a positive number:");

        let mut target = String::new();
        io::stdin()
            .read_line(&mut target)
            .expect("failed to read line");

        if target.trim() == "exit" {
            break;
        }
        let target = match target.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let start = Instant::now();
        let result = fibonacci_non_recursive(target);
        let elapsed_time = start.elapsed().as_micros();
        println!("Non-recursive took {:.3e} µs and the result is {}", elapsed_time, result);

        let start = Instant::now();
        let result = fibonacci_recursive(target);
        let elapsed_time = start.elapsed().as_micros();
        println!("Recursive took {:.3e} µs and the result is {}", elapsed_time, result);
    }
}

