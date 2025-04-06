fn main() {
    let mut sum = 0;

    for numbers in 1..=100 {
        sum += numbers;
    }

    println!("Sum is {}", sum)
}
