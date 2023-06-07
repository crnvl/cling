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