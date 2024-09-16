#[cfg(feature = "sanrs")]
use std::io::{self, Write, BufRead};
#[cfg(feature = "sanrs")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "sanrs")]
use crate::SanityClient;

#[cfg(feature = "sanrs")]
pub async fn run_shell(client:&SanityClient) {
    let mut client = Arc::new(Mutex::new(client));
    let mut input = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        print!("sanrs> ");
        io::stdout().flush().unwrap();
        handle.read_line(&mut input).unwrap();
        let mut input = String::from(input.trim());
        if input == "exit" {
            break;
        }
        let mut client = client.lock().unwrap();
        let input_str = input.as_str();
        let response = client.query().fetch(input_str).await;
        match response {
            Ok(resp) => {
                println!("{:?}", resp)
            },
            Err(err) => {
                eprintln!("The provided query returned an error: {:?}", err)
            }
        }
        input.clear();
    }
}
