use std::net::TcpStream;

pub fn connect() {
    if let Ok(stream) = TcpStream::connect("localhost:8080") {
        println!("Connection established!");
    } else {
        println!("Couldn't connect to the server")
    }
}
