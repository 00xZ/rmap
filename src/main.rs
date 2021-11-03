
extern crate rand;

use rand::thread_rng;
use rand::Rng;

fn main() {
        let a: i64 = rand::thread_rng().gen_range(0..255);
        let b: i64 = rand::thread_rng().gen_range(0..255);
        let c: i64 = rand::thread_rng().gen_range(0..255);
        let d: i64 = rand::thread_rng().gen_range(0..255);
        let ip = format!("{}.{}.{}.{}", a,b,c,d);
        println!("IP: {}", ip);
}
