use std::io::{self, Write, BufRead};
use std::sync::{Arc, Mutex};
#[cfg(feature = "sanrs")]
use crate::SanityClient;

pub fn run_shell(&client:SanityClient) {
    let mut client = Arc::new(Mutex::new(client));
    let mut input = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        print!("sanrs> ");
        io::stdout().flush().unwrap();
        handle.read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "exit" {
            break;
        }
        let mut client = client.lock().unwrap();
        let response = client.query(input);
        println!("{:?}", response);
        input.clear();
    }
}
