use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use lib_mfs::{
    reader::MfsStreamReader,
    response::MfsResponse,
    user::{MfsUser, MfsUsers},
    writer::MfsStreamWriter, command::{CommandParser, MfsCommandParser},
};
use rayon::ThreadPoolBuilder;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    let pool = ThreadPoolBuilder::new().num_threads(8).build().unwrap();
    let max_data_size = 1000;
    let break_up_data = true;
    let users = Arc::new(Mutex::new(MfsUsers::new("users")));
    users
        .lock()
        .unwrap()
        .add_user(MfsUser::new("nate", "12345"))
        .unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.install(|| handle_client(stream, Arc::clone(&users), max_data_size, break_up_data));
    }
}

fn handle_client(
    mut stream: TcpStream,
    users: Arc<Mutex<MfsUsers>>,
    max_data_size: u64,
    break_up_data: bool,
) {
    let writer = MfsStreamWriter::new(max_data_size, break_up_data);
    let reader = MfsStreamReader::new(max_data_size, break_up_data);

    // Write the max_data_size
    writer.write(&mut stream, &reader.max_data_size()).unwrap();
    // Write the break_up_data
    writer.write(&mut stream, &reader.break_up_data()).unwrap();

    // get login info
    let user = reader.read::<MfsUser>(&mut stream).unwrap();

    dbg!(&user);

    let mut response = MfsResponse::new();
    response.set_message(Some(String::from("Welcome")));
    match users.lock().unwrap().correct_password(&user) {
        Ok(b) => {
            if !b {
                response.set_failure();
                response.set_message(Some(String::from("password does not match")));
            }
        }
        Err(error) => {
            response.set_failure();
            response.set_message(Some(error.to_string()))
        }
    }

    writer.write(&mut stream, &response).unwrap();

    if !response.success() {
        return;
    }

    loop {
        // Read the users raw command
        let command = reader.read::<String>(&mut stream).unwrap();
        let command = CommandParser::parse(command.as_str()).unwrap();

        writer.write(&mut stream, &command.execute().unwrap()).unwrap();
    }
}
