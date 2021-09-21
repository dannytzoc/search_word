use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3{
        println!("Not enoough commands");
        println!("to look at all avaible commands type -help or -h ");

    }else {
        let command = &args[1];
        let query = &args[2];
        let filename = &args[3];
        commands(command,query,filename);

    }

}
fn word_counter(word:&String ,nm_file:&String){
    let content = fs::read_to_string(nm_file)
        .expect("Could not read in file");
    let wcount = content.matches(word).count();
    println!("The word appears {} times ",wcount );
}
fn commands(c:&String , q: &String, f: &String){
    if c == "c"{
        word_counter(q,f);
    }
}
