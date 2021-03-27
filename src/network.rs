use reqwest;
use rhai::plugin::*;

#[export_module]
pub mod network {
    pub fn download(url: &str) -> String {
        if let Ok(response) = reqwest::blocking::get(url) {
            return response.text().unwrap_or_default();
        }
        return String::new();
    }
}
