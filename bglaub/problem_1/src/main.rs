fn main() {
    const LIMIT :usize = 999;
    println!("{}", (sum_divisible_by(3, LIMIT) + sum_divisible_by(5, LIMIT) - sum_divisible_by(15, LIMIT)));
}

fn sum_divisible_by(divisor: usize, limit: usize) -> usize {
   let p = limit / divisor;
   (divisor * (p * (p + 1))) / 2
}
