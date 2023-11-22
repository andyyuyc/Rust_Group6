use std::io;

pub struct BehaviorHidingModule {
    user_interface: UserInterface,
    api: Api,
}

impl BehaviorHidingModule {
    pub fn new() -> Self {
        Self {
            user_interface: UserInterface::new(),
            api: Api::new(),
        }
    }

    pub fn execute_command(&self, command: &str) {
        if command.starts_with("cli:") {
            let cli_command = &command[4..]; 
            self.user_interface.handle_command(cli_command);
        } else if command.starts_with("web:") {
            let web_command = &command[4..]; 
            self.api.handle_request(web_command);
        } else {
            println!("Invalid command format.");
        }
    }
}

pub struct UserInterface;

impl UserInterface {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_command(&self, command: &str) {
        match command {
            "commit" => self.handle_commit(),
            "help" => self.handle_help(),
            _ => println!("Unknown command: {}", command),
        }
    }

    fn handle_commit(&self) {
        println!("Committing changes...");
    }

    fn handle_help(&self) {
        println!("Displaying help information...");
    }
}

pub struct Api;

impl Api {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_request(&self, request: &str) {
       
        println!("Handling web request: {}", request);
    }
}

fn main() {
    let behavior_module = BehaviorHidingModule::new();

    behavior_module.execute_command("cli:commit");
    behavior_module.execute_command("cli:help");
    behavior_module.execute_command("cli:list_files"); 
}
