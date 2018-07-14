extern crate primes;
use primes::Primes;

fn main() {

    let n: u64 = 600851475143;

    let primes = Primes::upto((int_sqrt(n) + 1) as usize);

    for &p in primes.list.iter().rev() {
        if is_divisor(n, p as u64) {
            println!("{}", p);
            break;
        }
    }
    
}

fn int_sqrt(n: u64) -> u64 {

    let mut m: u64 = 1;
    let mut old_m: u64;

    loop {
        old_m = m;
        m = (m + (n / m)) / 2;

        if m == old_m || m == old_m + 1 {
            break;
        }
    }

    m
}

fn is_divisor(n: u64, p: u64) -> bool {
    let m = n / p;
    m * p == n
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_int_sqrt() {
        assert_eq!(::int_sqrt(10), 3);
    }
}
