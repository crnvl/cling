use crate::util::read_properties;

pub fn get_debug() -> String {
    let base_url = read_properties().0;
    let response = reqwest::blocking::get(&format!("{}/", base_url)).expect("Unable to get response");
    response.text().expect("Unable to get text")
}