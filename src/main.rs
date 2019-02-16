use std::io;

fn main() {
    println!("Fibonacci Series");

    loop {
        println!("please input a number");

        let mut target = String::new();

        io::stdin()
            .read_line(&mut target)
            .expect("failed to read line");

        if target.trim() == "exit" {
            break;
        }
        let target: u32 = match target.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let result = get_fibonacci_non_recursive(target);
        println!("the result non recursive is {}", result);

        let result = get_fibonacci_recursive(target);
        println!("the result recursive is {}", result);
    }
}

fn get_fibonacci_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => get_fibonacci_recursive(n - 1) + get_fibonacci_recursive(n - 2),
    }
}

fn get_fibonacci_non_recursive(n: u32) -> u64 {
    match n {
        0 => return 0,
        1 => return 1,
        _ => {}
    }

    let mut n_2 = 0;
    let mut n_1 = 1;
    let mut sum = 1;
    // loop starting from 2
    for _i in 1..n {
        sum = n_2 + n_1;
        n_2 = n_1;
        n_1 = sum;
    }
    sum
}
