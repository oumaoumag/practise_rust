pub fn count_factorial_steps(factorial: u64) -> u64 {
    if factorial == 0 || factorial == 1 {
        return 0;
    }

    let mut count = 0;
    let mut product = 1;

    while product < factorial {
        count += 1;
        product *= count;
    }

    if factorial == product {
         count
    } else {
        0
    }
}