pub struct Fibonacci {
    limit: usize,
    curr: usize,
    prev: usize,
}

impl Fibonacci {
    pub fn new(limit: usize) -> Fibonacci {
        Fibonacci { 
            limit,
            curr: 1,
            prev: 0
        }
    }
}

impl Iterator for Fibonacci {

    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {

        let curr :usize = self.curr + self.prev;

        if curr < self.limit {
            self.prev = self.curr;
            self.curr = curr;
            Some(self.curr)
        } else {
            None
        }
    }
}
