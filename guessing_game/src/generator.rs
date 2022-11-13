use rand::prelude::*;

fn main() {
    let mut rng: ThreadRng = rand::thread_rng();

    let random_number = generate_float(&mut rng);
    println!("{}", random_number);
}

fn generate_float(generator: &mut ThreadRng) -> f64 {
    let placeholder: f64 = generator.gen();
    return placeholder * 10.0
}