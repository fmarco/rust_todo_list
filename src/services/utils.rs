use std::io;

pub fn read_from_stdin() -> String {
    let mut buffer = String::new();
    
    io::stdin()
    .read_line(&mut buffer)
    .expect("Error to read the std input");
    
    buffer
}

pub fn print_success_message() {
    println!("Done!");
}

pub fn read_string_from_stdin() -> String {
    read_from_stdin().as_str().trim().to_string()
}
