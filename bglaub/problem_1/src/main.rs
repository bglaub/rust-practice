fn main() {
    let mut sum = 0;

    for num in 1..1000 {
        sum += if num % 3 == 0 || num % 5 == 0 { num } else { 0 }
    }

    println!("{}", sum);
}
