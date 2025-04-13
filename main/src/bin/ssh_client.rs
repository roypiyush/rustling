use std::{
    io::{self, Write},
    sync::{Arc, Mutex},
    thread::{self},
};

use std::sync::mpsc;

use ssh::{self, TerminalSize};

///
/// This is just a demo. Not to be used in production.
///
fn main() {
    let mut cmd_args = std::env::args();
    cmd_args.next(); // skip this file

    let host_with_port = cmd_args.next();
    let username = cmd_args.next();
    let password = cmd_args.next();
    let prompt = cmd_args.next(); // prompt could be auto detected

    let reader = thread::Builder::new().name("reader".to_string());
    let writer = thread::Builder::new().name("writer".to_string());

    match (&host_with_port, &username, &password, &prompt) {
        (Some(_), Some(_), Some(_), Some(_)) => {
            // correct input. program will proceed
        }
        (_, _, _, _) => {
            println!("Usage: ./ssh_client <host:port> <username> <password> <command> <prompt>");
            return;
        }
    }

    let prompt = prompt.unwrap();

    let mut session: ssh::SessionBroker = ssh::create_session()
        .username(&username.unwrap())
        .password(&password.unwrap())
        .connect(&host_with_port.unwrap())
        .unwrap()
        .run_backend();

    let tv = TerminalSize::from(80, 40);
    let channel = Arc::new(Mutex::new(session.open_shell_terminal(tv).unwrap()));

    let write_channel = Arc::clone(&channel);
    let read_channel = Arc::clone(&channel);

    let is_prompt = Arc::new(Mutex::new(false));
    let is_exit = Arc::new(Mutex::new(false));
    let is_exit_reader = Arc::clone(&is_exit);
    let is_exit_writer = Arc::clone(&is_exit);

    let (tx, rx) = mpsc::channel();

    let join_handle_reader = reader
        .spawn(move || loop {
            if *is_prompt.try_lock().unwrap() {
                rx.recv().unwrap();
                if *is_exit_reader.try_lock().unwrap() {
                    break;
                }

                *is_prompt.try_lock().unwrap() = false;
            } else {
                let data = read_channel.try_lock().unwrap().read().unwrap();
                let string_data = String::from_utf8(data).unwrap();

                print!("{}", string_data);
                io::stdout().flush().unwrap();

                // Idea is that prompt marks end of output
                if string_data.contains(&prompt) {
                    *is_prompt.try_lock().unwrap() = true;
                }
            }
        })
        .unwrap();

    let join_handle_writer = writer
        .spawn(move || loop {
            if *is_exit_writer.try_lock().unwrap() {
                break;
            }

            let mut cmd = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut cmd).unwrap();

            if let Ok(mut write) = write_channel.try_lock() {
                if cmd.contains("exit") {
                    *is_exit_writer.try_lock().unwrap() = true;
                }
                write.write(cmd.as_bytes()).unwrap();
                tx.send(cmd).unwrap();
            } else {
                // Error Scenarios [add more as required]
                // 1. Multiple line command where prompt disappears
                println!("Error Occured due to locking in reader thread. A bug is found.");
            }
        })
        .unwrap();

    join_handle_writer.join().unwrap();
    join_handle_reader.join().unwrap();
    session.close();
}
