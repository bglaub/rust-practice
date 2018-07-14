pub struct Primes {
    pub list: Vec<usize>
}

impl Primes {
    pub fn upto(limit: usize) -> Primes {

        let mut is_prime : Vec<bool> = Vec::with_capacity(limit);

        // push false for 0 and 1
        is_prime.push(false);
        is_prime.push(false);

        for i in 2..limit {
            is_prime.push(true);
        }

        let mut list : Vec<usize> = Vec::new();

        for i in 2..limit {
            if is_prime[i] {
                list.push(i);
                let mut j = 2 * i;
                while j < limit {
                    is_prime[j] = false;
                    j += i;
                }
            }
        }
        
        Primes {
            list
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_primes() {
        let a = [2, 3, 5, 7];
        let primes = ::Primes::upto(10);
        for (i, &p) in primes.list.iter().enumerate() {
            assert_eq!(p, a[i]);
        }
    }
}
