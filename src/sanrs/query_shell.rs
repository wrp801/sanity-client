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
use rustyline::DefaultEditor;
#[cfg(feature = "sanrs")]
use rustyline::error::ReadlineError;
#[cfg(feature = "sanrs")]
use serde_json::Value;

#[cfg(feature = "sanrs")]
// Function to recursively format JSON with color
fn colorize_json(values: &[Value], indent: usize) -> String {
    let mut result = String::new();
    let indent_str = "  ".repeat(indent); // Indentation for pretty formatting

    for value in values {
        match value {
            Value::Object(map) => {
                result.push_str(&format!("{{\n"));
                for (key, value) in map {
                    result.push_str(&format!(
                        "{}{}: {}\n", 
                        indent_str, 
                        key.blue().bold(), 
                        colorize_json(&[value.clone()], indent + 1) // Recursively colorize the value
                    ));
                }
                result.push_str(&format!("{}}}", indent_str));
            }
            Value::Array(arr) => {
                result.push_str(&format!("[\n"));
                result.push_str(&colorize_json(arr, indent + 1)); // Recursively handle arrays
                result.push_str(&format!("{}]", indent_str));
            }
            Value::String(s) => {
                result.push_str(&format!("{}", s.cyan().bold()));
            }
            Value::Number(num) => {
                result.push_str(&format!("{}", num.to_string().yellow()));
            }
            Value::Bool(b) => {
                if *b {
                    result.push_str(&format!("{}", "true".cyan()));
                } else {
                    result.push_str(&format!("{}", "false".cyan()));
                }
            }
            Value::Null => {
                result.push_str(&format!("{}", "null".red()));
            }
        }
        result.push(',');
        result.push('\n');
    }

    result
}


#[cfg(feature = "sanrs")]
// TODO: fill out docs

pub async fn run_shell(sanity_client: &SanityClient) {
        let client = Arc::new(Mutex::new(sanity_client));
    
    // Create a rustyline editor instance
    let mut rl = DefaultEditor::new().unwrap();
    
    loop {
        // Prompt with "sanrs> "
        let readline = rl.readline(&format!("{}", "sanrs> ".bright_green().bold()));
        
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());

                let input = line.trim();
                if input == "exit" {
                    break;
                }
                if input == "dataset" {
                    println!("Using dataset {}", sanity_client.dataset);
                    continue;
                }
                
                let mut client = client.lock().unwrap();
                let response = client.query().fetch(input).await;

                match response {
                    Ok(resp) => {
                        // Pretty print the JSON response with color
                        // let pretty_string = serde_json::to_string_pretty(&resp.result).unwrap();
                        // println!("{}", pretty_string);
                        let colored_json = colorize_json(&resp.result, 1);
                        println!("{}", colored_json);
                    }
                    Err(err) => {
                        eprintln!("The provided query returned an error: {:?}", err);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("Error reading line: {:?}", err);
                break;
            }
        }
    }
}
