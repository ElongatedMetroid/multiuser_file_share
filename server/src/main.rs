use std::net::{TcpListener, TcpStream};

use lib_mfs::MfsStreamReader;
use rayon::ThreadPoolBuilder;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    let pool = ThreadPoolBuilder::new().num_threads(8).build().unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.install(|| handle_client(stream));
    }
}

fn handle_client(mut stream: TcpStream) {
    println!("{}", MfsStreamReader::read::<String>(&mut stream).unwrap());
}
