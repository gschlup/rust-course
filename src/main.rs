use rand::{thread_rng, Rng};
fn main() {
    let x = thread_rng().gen_range(1..100);

    println!("{}",x);
}
