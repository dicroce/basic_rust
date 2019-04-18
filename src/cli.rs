use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    if args.len() > 1 {
        let cmd = args[1].clone();
        if cmd == "hello" {
            println!("hello, how are you?");
        }
    }
}
