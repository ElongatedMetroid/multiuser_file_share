use std::net::{TcpListener, TcpStream};

use lib_mfs::{reader::MfsStreamReader, user::MfsUser, writer::MfsStreamWriter};
use rayon::ThreadPoolBuilder;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    let pool = ThreadPoolBuilder::new().num_threads(8).build().unwrap();
    let max_data_size = 1000;
    let break_up_data = true;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.install(|| handle_client(stream, max_data_size, break_up_data));
    }
}

fn handle_client(mut stream: TcpStream, max_data_size: u64, break_up_data: bool) {
    let writer = MfsStreamWriter::new(max_data_size, break_up_data);
    let reader = MfsStreamReader::new(max_data_size, break_up_data);

    // Write the max_data_size
    writer.write(&mut stream, &reader.max_data_size()).unwrap();
    // Write the break_up_data
    writer.write(&mut stream, &reader.break_up_data()).unwrap();

    // get login info
    reader.read::<MfsUser>(&mut stream).unwrap();
}
