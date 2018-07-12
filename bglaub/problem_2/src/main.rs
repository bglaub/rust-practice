extern crate euler;
use euler::fibonacci::Fibonacci;

fn main() {
    println!("{}", Fibonacci::new(4000000).filter(|x| x % 2 == 0).sum::<usize>());
}
