use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    println!("Número aleatorio: {}", rng.gen_range(0..10));
}
