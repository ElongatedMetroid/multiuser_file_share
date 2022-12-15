use std::net::TcpStream;

use lib_mfs::MfsStreamWriter;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:6969").unwrap();

    handle_stream(stream);
}

fn handle_stream(mut stream: TcpStream) {
    MfsStreamWriter::write(&mut stream, &String::from("Hello World8888")).unwrap();
}
