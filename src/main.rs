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
                let json = api::get_boards();

                let boards: Vec<String> = serde_json::from_str(&json).unwrap();
                let mut output = String::new();
                for board in boards.iter() {
                    output.push_str(&format!("{}\n", board));
                }
                output
            }
            "exit" => break,
            _ => "Unknown command".to_string(),
        };
        println!("{}\n", output);
    }

    println!("Goodbye!");
}
