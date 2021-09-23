use std::env; //args
use std::fs; //file library

fn main() {
    let args: Vec<String> = env::args().collect(); //collect the args and puts it inside a vector
    if args.len() < 3{ //get the command line arugemnts for error checkign
        println!("Not enoough commands");
        help();

    }else {
        let command = &args[1]; //get the command that user want to put
        let query = &args[2];
        let filename = &args[3];
        commands(command,query,filename); //commands function

    }

}
fn word_counter(word:&String ,nm_file:&String){
    let content = fs::read_to_string(nm_file)
        .expect("Could not read in file"); //read in the files
    let wcount = content.matches(word).count(); //count the word that we are searching for
    println!("The word appears {} times ",wcount );
}
fn help(){ //function to print out the help
    println!("command below currently waiting to add more commands ");
    println!("-c \t \t \t count the number of time a word appears in a txt file");
}
fn commands(c:&String , q: &String, f: &String){ //commands function to keep track of the type of commands you want
    if c == "c" || c=="count"{
        word_counter(q,f);
    }
    else if c == "help" || c == "h"{
        help()
    }else{
        println!("Not a commnad made yet");
    }
}
