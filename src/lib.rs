use rand::Rng;

pub fn greet(name: &str) {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1..=10);
    println!("Hello, {}! Your lucky number is {}", name, number);
}