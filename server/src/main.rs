use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
};

use lib_mfs::{
    command::{CommandParser, MfsCommandParser},
    reader::MfsStreamReader,
    response::MfsResponse,
    user::{MfsUser, MfsUsers},
    writer::MfsStreamWriter,
};
use rayon::ThreadPoolBuilder;
use server::config::Config;

fn main() {
    let config = Config::load("server.conf").unwrap();

    let listener = TcpListener::bind(config.addr()).unwrap();
    let pool = ThreadPoolBuilder::new().num_threads(config.threads()).build().unwrap();

    let users = Arc::new(Mutex::new(MfsUsers::new(config.user_data_filename())));
    users
        .lock()
        .unwrap()
        .add_user(MfsUser::new("nate", "12345"))
        .unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.install(|| handle_client(stream, Arc::clone(&users), &config));
    }
}

fn handle_client(
    mut stream: TcpStream,
    users: Arc<Mutex<MfsUsers>>,
    config: &Config,
) {
    let writer = MfsStreamWriter::new(config.max_data_size(), config.break_up_data());
    let reader = MfsStreamReader::new(config.max_data_size(), config.break_up_data());

    // Write the max_data_size
    if let Err(error) = writer.write(&mut stream, &reader.max_data_size()) {
        eprintln!("({error}) write to client `{:?}` failed, killing ...", stream.peer_addr());
        return;
    }
    // Write the break_up_data
    if let Err(error) = writer.write(&mut stream, &reader.break_up_data()) {
        eprintln!("({error}) write to client `{:?}` failed, killing ...", stream.peer_addr());
        return;
    }

    let mut response = MfsResponse::new();
    response.set_message(Some(String::from("Welcome")));

    // get login info
    match reader.read::<MfsUser>(&mut stream) {
        Ok(user) => {
            match users.lock().unwrap().correct_password(&user) {
                Ok(b) => {
                    if !b {
                        response.set_failure_with_message(Some(String::from("password does not match")));
                    }
                }
                Err(error) => {
                    response.set_failure_with_message(Some(error.to_string()));
                }
            }
        },
        Err(error) => {
            response.set_failure_with_message(Some(error.to_string()));
        }
    }

    if let Err(error) = writer.write(&mut stream, &response) {
        eprintln!("({error}) write to client `{:?}` failed, killing ...", stream.peer_addr());
        return;
    }

    if !response.success() {
        return;
    }

    loop {
        let mut response = MfsResponse::new();

        // Read the users raw command
        match reader.read::<String>(&mut stream) {
            Ok(command) => {
                match CommandParser::parse(command.as_str()) {
                    Ok(command) => {
                        match command.execute() {
                            Ok(command_response) => {
                                response = command_response;
                            },
                            Err(error) => {
                                response.set_failure_with_message(Some(error.to_string()));
                            },
                        };
                    },
                    Err(error) => {
                        response.set_failure_with_message(Some(error.to_string()));
                    },
                };
            },
            Err(error) => {
                response.set_failure_with_message(Some(error.to_string()));
            },
        }
        
        if let Err(error) = writer.write(&mut stream, &response) {
            eprintln!("({error}) write to client `{:?}` failed, killing ...", stream.peer_addr());
            return;
        }
    }
}
