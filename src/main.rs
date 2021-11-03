extern crate rand;

use rand::thread_rng;
use rand::Rng;

fn main() {
    let mut rng = thread_rng();
    let distr = rand::distributions::Uniform::new_inclusive(1, 100);
    let mut nums = [0i32; 3];
    for x in &mut nums {
        *x = rng.sample(distr);
    }
    println!("Some numbers: {:?}", nums);
}
