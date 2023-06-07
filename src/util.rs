use std::io::Write;

use serde::Deserialize;

#[derive(Deserialize)]
struct JsonProps {
    base_url: String,
    username: String,
}

pub fn read_properties() -> (String, String) {
    let content = std::fs::read_to_string("./config.json").expect("Unable to read file");
    let json: JsonProps = serde_json::from_str(&content).expect("JSON was not well-formatted");

    (json.base_url, json.username)
}

pub fn read_console(prefix: &str) -> String {
    print!("{}", prefix);
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()

}