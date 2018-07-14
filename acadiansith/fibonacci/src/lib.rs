pub struct Fibonacci {
    curr: u32,
    prev: u32,
    limit: u32
}

impl Fibonacci {
    pub fn upto(limit: u32) -> Fibonacci {
        Fibonacci {
            curr: 1,
            prev: 0,
            limit
        }
    }
}

impl Iterator for Fibonacci {

    type Item = u32;

    fn next(&mut self) -> Option<u32> {

        if self.curr > self.limit {
            return None;
        }

        let tmp = self.curr;
        self.curr += self.prev;
        self.prev = tmp;

        Some(tmp)
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fibonacci() {
        let a = [1, 1, 2, 3];
        let fib = ::Fibonacci::upto(a.len() as u32);
        for (i, n) in fib.enumerate() {
            assert_eq!(n, a[i]);
        }
    }
}
