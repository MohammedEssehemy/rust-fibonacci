#[no_mangle]
pub extern fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

#[no_mangle]
pub extern fn fibonacci_non_recursive(n: u32) -> u64 {
    match n {
        0 => return 0,
        1 => return 1,
        _ => {
            // start by i = 2
            let mut f_i_2 = 0; // f(i-2) = f(0) = 0
            let mut f_i_1 = 1; // f(i-1) = f(1) = 1
            // loop starting from 2 and to n inclusive
            for _i in 2..=n {
                let sum = f_i_2 + f_i_1;
                f_i_2 = f_i_1;
                f_i_1 = sum;
            }

            return f_i_1;
        }
    }
}
