// main.rs
use tokio::runtime;
mod init;
mod clone;
mod status;
mod errorhandling;

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
        "status" => {
            if let Err(e) = status::status().await {
                println!("Error: {}", e);
            }
        },
        _ => println!("Unknown command"),
        
    }
}