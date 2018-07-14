extern crate fibonacci;

fn main() {

    let fib = fibonacci::Fibonacci::upto(4_000_000);

    let mut s = 0;

    for n in fib {
        if n % 2 == 0 {
            s += n;
        }
    }

    println!("{}", s)

}
