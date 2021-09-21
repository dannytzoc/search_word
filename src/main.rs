use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3{
        println!("Not enoough commands");
        println!("to look at all avaible commands type -help or -h ");

    }else {
        let query = &args[1];
        let filename = &args[2];

        println!("Searching for {}", query);
        println!("Size of vec {}",args.len());
        println!("In file {}", filename);
    }

}
