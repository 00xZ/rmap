extern crate rand;
extern crate port_scanner;
use port_scanner::scan_port_addr;
use rand::thread_rng;
use rand::Rng;
fn scan(mut param_target:String) {
        println!("IP: {}", param_target);
	    let tester = scan_port_addr(param_target);
        println!("{}", tester);
        if tester == false {
            println!("Closed");
        }
        else {
            println!("OPEN");
        }
}
fn main() {
        let a: i64 = rand::thread_rng().gen_range(0..255);
        let b: i64 = rand::thread_rng().gen_range(0..255);
        let c: i64 = rand::thread_rng().gen_range(0..255);
        let d: i64 = rand::thread_rng().gen_range(0..255);
        let target = format!("{}.{}.{}.{}:80", a,b,c,d);
        //println!("IP: {}", target);
        scan(target);
}
