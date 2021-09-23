use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3{
        println!("Not enoough commands");
        help();

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
fn help(){
    println!("command below currently waiting to add more commands ");
    println!("-c \t \t \t count the number of time a word appears in a txt file");
}
fn commands(c:&String , q: &String, f: &String){
    if c == "c" || c=="count"{
        word_counter(q,f);
    }
    else if c == "help" || c == "h"{
        help()
    }else{
        println!("Not a commnad made yet");
    }
}
