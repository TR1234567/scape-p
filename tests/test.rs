use std::fs::File;
use std::io::Write;
use std::string::String;

use dotenv::dotenv;
use serde_json::Value;
use tokio;

#[tokio::main]
async fn read_data() {
    dotenv().ok();
    let key = std::env::var("API_KEY").expect("API_KEY must be set.");
    let url = format!(
        "https://api.stlouisfed.org/fred/releases?api_key={}&file_type=json&limit=3",
        key
    );

    let client = reqwest::Client::new();
    let response = client.get(url).send().await.unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            println!("Success! {:?}", response);
            let json = response.text().await.unwrap();
            let json_str: &str = string_to_static_str(json);
            let json_data: Value = serde_json::from_str(json_str).expect("Failed to parse JSON");
            print!("{:?}", json_data["releases"]);
            write_json_to_file("output.json", &json_data).expect("Failed to write JSON to file");
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        _ => {
            panic!("Uh oh! Something unexpected happened.");
        }
    };
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn write_json_to_file(file_path: &str, data: &Value) -> std::io::Result<()> {
    // Serialize the JSON data to a string
    let json_str = serde_json::to_string(data)?;

    // Open a file for writing
    let mut file = File::create(file_path)?;

    // Write the JSON string to the file
    file.write_all(json_str.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_read_data() {
        // You can write test cases for your read_data function here
        // For example, you can use a mock HTTP server or simulate different API responses
        // and test how your function handles them.
    }

    #[test]
    fn test_string_to_static_str() {
        let s = String::from("hello");
        let static_str = string_to_static_str(s);
        assert_eq!(static_str, "hello");
    }

    #[test]
    fn test_write_json_to_file() {
        // You can write test cases for your write_json_to_file function here
        // For example, you can create a temporary file, write JSON data to it,
        // and then read the file to verify its contents.
    }
}

fn main() {
    read_data()
}
