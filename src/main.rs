
extern crate rand;
extern crate port_scanner;
use port_scanner::scan_port_addr;
use rand::Rng;
use std::borrow::Borrow;
use std::fs::OpenOptions;
use std::io::Write;
use std::fs::File;
fn scan(param_target:String) {
	    File::create("list.txt");
        let mut file = OpenOptions::new().append(true).open("list.txt").expect("Cant open file check permissions.");
        println!("IP: {}", param_target);
	    let tester = scan_port_addr(&param_target);
        println!("{}", tester);
        if tester == false {
            println!("Closed");
        }
        else {
            println!("OPEN");
            file.write(param_target.as_bytes());
        }
}
fn genip() {
        let a: i64 = rand::thread_rng().gen_range(0..255);
        let b: i64 = rand::thread_rng().gen_range(0..255);
        let c: i64 = rand::thread_rng().gen_range(0..255);
        let d: i64 = rand::thread_rng().gen_range(0..255);
        let target = format!("{}.{}.{}.{}:80", a,b,c,d);
        //println!("IP: {}", target);
        scan(target);
}
fn main() {
	let mut count = 0u32;
	loop {
		count += 1;
        genip();
        if count == 10 {
			break;
        }
	}
}
