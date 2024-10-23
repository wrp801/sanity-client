#[cfg(feature = "sanrs")]
use crate::SanityClient;
#[cfg(feature = "sanrs")]
use std::io::{self, BufRead, Write};
#[cfg(feature = "sanrs")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "sanrs")]
extern crate serde_json;
#[cfg(feature = "sanrs")]
use colored::*;

#[cfg(feature = "sanrs")]
// TODO: fill out docs

pub async fn run_shell(sanity_client: &SanityClient) {
    let mut client = Arc::new(Mutex::new(sanity_client));
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    loop {
        print!("{} ", "sanrs> ".bright_green().bold());
        let mut input = String::new();
        io::stdout().flush().unwrap();
        handle.read_line(&mut input).unwrap();
        let mut input = String::from(input.trim());
        if input == "exit" {
            break;
        }
        if input == "dataset" {
            println!("Using dataset {}", sanity_client.dataset);
            continue
        }
        let mut client = client.lock().unwrap();
        let input_str = input.as_str();
        let response = client.query().fetch(input_str).await;
        match response {
            Ok(resp) => {
                // TODO: It would be great to have colored output like jq
                let pretty_string = serde_json::to_string_pretty(&resp.result).unwrap();
                println!("{}", pretty_string);
                input.clear()
            }
            Err(err) => {
                eprintln!("The provided query returned an error: {:?}", err);
                input.clear()
            }
        }
    }
}
