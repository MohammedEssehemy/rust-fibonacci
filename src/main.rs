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

fn get_fibonacci_recursive(n:u32) -> u64 {
	match n {
		0     => panic!("zero is not a right argument!"),
		1 | 2 => 1,
		3     => 2,
        _     => get_fibonacci_recursive(n - 1) + get_fibonacci_recursive(n - 2)
    }
}

fn get_fibonacci_non_recursive(n:u32) -> u64 {
    if n == 0 {
        panic!("zero is not a right argument!");
    } else if n == 1 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _i in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}
