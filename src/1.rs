// Generate a random integer between 1 and 10
let mut rng = rand::thread_rng();
let x = rng.gen_range(1..=10);

// Use the random number to determine a random color
let colors = ["red", "green", "blue"];
let color = colors[x as usize - 1];
