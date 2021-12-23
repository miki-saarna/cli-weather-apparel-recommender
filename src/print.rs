use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let name = args[1].clone();

    if name == "Brad" {
        println!("Hi {}, How's it going?", name)
    } else {
        println!("nope...");
    }

}