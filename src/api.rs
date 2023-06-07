use crate::util::read_properties;

pub fn get_debug() -> String {
    let base_url = read_properties().0;
    let response = reqwest::blocking::get(&format!("{}/", base_url)).expect("Unable to get response");
    response.text().expect("Unable to get text")
}

pub fn get_boards() -> String {
    let base_url = read_properties().0;
    let response = reqwest::blocking::get(&format!("{}/boards", base_url)).expect("Unable to get response");
    let res_text = response.text().expect("Unable to get text");
    res_text
}

pub fn get_posts(board: &str) -> String {
    let base_url = read_properties().0;
    let response = reqwest::blocking::get(&format!("{}/posts/{}", base_url, board)).expect("Unable to get response");
    let res_text = response.text().expect("Unable to get text");
    res_text
}

pub fn get_post(id: i64) -> String {
    let base_url = read_properties().0;
    let response = reqwest::blocking::get(&format!("{}/post/{}", base_url, id)).expect("Unable to get response");
    let res_text = response.text().expect("Unable to get text");
    res_text
}

pub fn get_comments(id: i64) -> String {
    let base_url = read_properties().0;
    let response = reqwest::blocking::get(&format!("{}/post/{}/comments", base_url, id)).expect("Unable to get response");
    let res_text = response.text().expect("Unable to get text");
    res_text
}

pub fn create_post(thumb_url: Option<String>, content: String, ref_id: Option<i64>, board: String) -> String {
    let base_url = read_properties().0;
    let username = read_properties().1;
    let client = reqwest::blocking::Client::new();

    let mut map = serde_json::Map::new();
    map.insert("username".to_string(), serde_json::Value::String(username));
    map.insert("content".to_string(), serde_json::Value::String(content));
    if let Some(thumb_url) = thumb_url {
        map.insert("thumb_url".to_string(), serde_json::Value::String(thumb_url));
    }
    if let Some(ref_id) = ref_id {
        map.insert("ref_id".to_string(), serde_json::Value::Number(serde_json::Number::from(ref_id)));
    }

    let response = client.post(&format!("{}/posts/{}", base_url, board))
        .json(&map)
        .send()
        .expect("Unable to get response");
    let res_text = response.text().expect("Unable to get text");
    res_text
}