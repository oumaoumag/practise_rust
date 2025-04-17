pub fn next_prime(nbr: u64) -> u64 {
    let mut prime = nbr;

    while !is_prime(prime) {
        prime += 1;
    }
    prime
}
   

fn is_prime(n : u64) -> bool {
    if n <= 1 { return false; }
    if n <= 3 { return true; }

    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let sqrt = (n as f64).sqrt() as u64;

    let mut i = 5; 

    while i <  sqrt {
        if n % i == 0 || n % (i + 2 ) == 0 {
            return false;
        }
        i += 6;
    }
    true
}