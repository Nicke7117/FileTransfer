use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::net::TcpStream;

fn get_file() -> File {
    loop {
        let mut filepath = String::new();
        println!("Enter the filepath: ");
        io::stdin()
            .read_line(&mut filepath)
            .expect("Failed to read line");
        let file = File::open(filepath.trim());
        match file {
            Ok(file) => {
                return file;
            }
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    println!("The file: {} was not found", filepath);
                    continue;
                }
                other_error => panic!("There was a problem opening the file: {:?}", other_error),
            },
        };
    }
}

fn get_message() -> String {
    let mut message = String::new();
    println!("Enter the message: ");
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read line");
    message.trim().to_string()
}

pub fn connect() {
    if let Ok(stream) = TcpStream::connect("localhost:8080") {
        println!("Connection established!");
    } else {
        println!("Couldn't connect to the server")
    }
}
