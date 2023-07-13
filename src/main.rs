mod network;

fn main() {
    let field = network::grid();
    let iterations = 1000;
    let learning_rate = 0.1;
    let discount_factor = 0.9;
    let max_steps = 100;
    let training = network::train(&field, iterations, learning_rate, discount_factor, max_steps);
    println!("{:#?}", training.unwrap());
}
