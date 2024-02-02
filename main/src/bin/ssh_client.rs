use std::{
    io::{self, Write}, sync::{Arc, Mutex}, thread::{self}
};


use ssh::{self, TerminalSize};

fn main() {
    
    let mut cmd_args = std::env::args();
    cmd_args.next(); // skip this file

    let host_with_port = cmd_args.next();
    let username = cmd_args.next();
    let password = cmd_args.next();

    let reader = thread::Builder::new().name("reader".to_string());
    let writer = thread::Builder::new().name("writer".to_string());

    match (&host_with_port, &username, &password) {
        (Some(_), Some(_), Some(_)) => {
            // correct input. program will proceed
        }
        (_, _, _) => {
            println!("Usage: ./ssh_client <host:port> <username> <password> <command>");
            return;
        }
    }

    let mut session: ssh::SessionBroker = ssh::create_session()
        .username(&username.unwrap())
        .password(&password.unwrap())
        .connect(&host_with_port.unwrap())
        .unwrap()
        .run_backend();


    let tv = TerminalSize::from(80, 40);
    let channel = Arc::new(Mutex::new(session.open_channel().unwrap().shell(tv).unwrap()));

    let write_channel = Arc::clone(&channel);
    let read_channel = Arc::clone(&channel);

    reader.spawn(move || loop {

        let data = read_channel.try_lock().unwrap().read().unwrap();
        print!("{}", String::from_utf8(data).unwrap());
        io::stdout().flush().unwrap();
        
    }).unwrap();

    let join_handle = writer.spawn(move || loop {
        let mut command = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut command).unwrap();
        //write_channel.try_lock().unwrap().write(command.as_bytes()).unwrap();
        if let Ok(mut write) = write_channel.try_lock() {
            write.write(command.as_bytes()).unwrap();
        } else {
            println!("Error Occured due to locking. Fix this to make it work");
        }
    }).unwrap();

    join_handle.join().unwrap();
}
