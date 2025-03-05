use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let x: u32 = rng.gen();
    let y: u32 = rng.gen();

    match x.cmp(&y) {
        Ordering::Less => println!("{} is less than {}", x, y),
        Ordering::Greater => println!("{} is greater than {}", x, y),
        Ordering::Equal => println!("{} is equal to {}", x, y),
    }
}
