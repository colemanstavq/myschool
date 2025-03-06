fn main() {
    let mut numbers = Vec::new();
    let mut sum = 0;

    for i in 1..10 {
        numbers.push(i);
    }

    for number in &numbers {
        sum += number;
    }

    println!("The sum of the first 10 numbers is: {}", sum);
}
