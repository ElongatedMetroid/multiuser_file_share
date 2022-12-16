use std::net::TcpStream;

use lib_mfs::{reader::MfsStreamReader, user::MfsUser, writer::MfsStreamWriter};

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

    dbg!(writer.max_data_size(), writer.break_up_data());

    writer
        .write(&mut stream, &MfsUser::new("bob", "12345"))
        .unwrap();
}
