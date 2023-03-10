use std::io;

fn main() {
    println!("Hey there! What's your name?");
    let user_name = read_line().unwrap();
    println!("Hey {}. Welcome to my rust program :)", user_name);
    println!("Bye!");
}

fn read_line() -> io::Result<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            trim_newline(&mut input);
            Ok(input)
        }
        Err(e) => Err(e),
    }
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
