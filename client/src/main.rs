use std::{net::TcpStream, io};

use lib_mfs::{reader::MfsStreamReader, user::MfsUser, writer::MfsStreamWriter, response::MfsResponse};

fn main() {
    let stream = TcpStream::connect("127.0.0.1:6969").unwrap();

    handle_stream(stream);
}

fn handle_stream(mut stream: TcpStream) {
    let mut reader = MfsStreamReader::new(8, false);
    let writer = MfsStreamWriter::new(
        reader.read::<u64>(&mut stream).unwrap(),
        reader.read::<bool>(&mut stream).unwrap(),
    );
    reader.set_max_data_size(writer.max_data_size());
    reader.set_break_up_data(writer.break_up_data());

    println!("Successfully Connected to {}", stream.peer_addr().unwrap());
    let mut username = String::new();
    println!("Enter your username: ");
    io::stdin().read_line(&mut username).unwrap();
    let mut password = String::new();
    println!("Enter your password: ");
    io::stdin().read_line(&mut password).unwrap();
    let user = MfsUser::new(username.trim(), password.trim());

    // Write the login info
    writer.write(&mut stream, &user).unwrap();
    // Read the response
    let response = reader.read::<MfsResponse>(&mut stream).unwrap();

    if !response.success() {
        println!("Login FAILED: {:?}", response.message());
    }

    println!("Server says: {:?}", response.message());

    loop {
        
    }
}
