use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("I am groot");
        }
        2 => {
            println!("->  {}", args[1]);
            if args[1] == "add" {
                println!("add");
            } else if args[1] == "expenses" {
                println!("expenses");
            }
        }
        _ => panic!("Too many arguments!"),
    }
    println!("args {:?}", args);
}
