// main.rs


mod init;
mod clone;
mod status;
mod errorhandling;
mod log;
mod file_management;

use crate::{file_management::{commit::commit, hash::DVCSHash}, state_management::checkout::checkout};
use crate::file_management::commit; 
use crate::file_management::commit::Commit; 

mod interface;
mod state_management;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: dvcs <command>");
        return;
    }

    match args[1].as_str() {
        "init" => init::init(),
        "clone" => clone::clone(),
        "errorhandling" => errorhandling::errorhandling(),
        "log" => {
            if let Err(e) = log::log() {
                println!("Error logging commit: {}", e);
            }
        },
        
        "status" => {
            if let Err(e) = status::status().await {
                println!("Error: {}", e);
            }
        },
        
        _ => println!("Unknown command"),
        
    }
}