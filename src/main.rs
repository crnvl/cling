use crate::api::get_debug;

mod api;
mod util;

fn main() {
    println!("cling v0.1.0; CLI client for Ping; edit ./config.json to change your settings;\n");

    loop {
        let input = util::read_console();
        let output = match input.as_str() {
            "debug" => get_debug(),
            "boards" => {
                api::get_boards()
            }
            "exit" => break,
            _ => "Unknown command".to_string(),
        };
        println!("{}\n", output);
    }

    println!("Goodbye!");
}
