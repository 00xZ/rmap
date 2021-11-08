

extern crate rand;
extern crate port_scanner;
use port_scanner::scan_port_addr;
use rand::Rng;
use std::borrow::Borrow;
use std::fs::OpenOptions;
use std::io::Write;
use std::thread;
use std::time::Duration;
use std::fs::File;
fn scan(param_target:String) {
        let mut file = OpenOptions::new().append(true).open("list.txt").expect("Cant open file check permissions.");
        println!("IP: {}", param_target);
	    let tester = scan_port_addr(&param_target);
        if tester == false {
            println!("Closed");
        }
        else {
            println!("OPEN");
            file.write(param_target.as_bytes());
        }
}

fn main() {
	let mut count = 0u32;
	loop {
		count += 1;
		let a: i64 = rand::thread_rng().gen_range(0..255);
        let b: i64 = rand::thread_rng().gen_range(0..255);
        let c: i64 = rand::thread_rng().gen_range(0..255);
        let d: i64 = rand::thread_rng().gen_range(0..255);
        let target = format!("{}.{}.{}.{}:80", a,b,c,d);
        thread::spawn(|| { scan(target); });
        thread::sleep(Duration::from_millis(5));
    if count == 10 {break;}
    }
	
}
