use chrono;
use reqwest::StatusCode;
use scraper:: { Html, Selector };
use std::fs::File;
use std::io::Write;

mod models;
mod utils;

fn main() {
    get_site();
}


#[tokio::main]
async fn get_site() {
    println!("Enter a domain name:");
    let mut domain = String::new();
    std::io::stdin().read_line(&mut domain).expect("Please enter a valid domain");
    println!("You entered {}", domain);
    let st = chrono::Local::now();
    let start_time = st.format("%Y-%m-%d_%H.%M.%S");
    println!("Start Time: {}", start_time);
    let client = utils::get_client();
    let filename = format!("{}.csv", domain);
    let mut writer = File::create(&filename).unwrap();
    let url = format!("https://{}", &domain);
    let result = client.get(url).send().await.unwrap();
    let dt = chrono::Local::now();
    println!("{}", dt.format("%Y-%m-%d_%H.%M.%S"));
    let raw_html = match result.status() {
    StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("Something went wrong!"),
        };

    let site_data: Vec<models::Site_Data> = Vec::new();

    let document = Html::parse_document(&raw_html);
    let selector = Selector::parse("a").unwrap();

    for element in document.select(&selector) {
        let inner = element.inner_html().to_string();
        let href = match element.value().attr("href") {
            Some(target_url) => target_url,
            _ => "no url found"
        };
        writeln!(
            &mut writer,
            "{},{}",
            &serde_json::to_string(&inner).unwrap(),
            &serde_json::to_string(&href).unwrap());
    }
    let et = chrono::Local::now();
    let end_time = et.format("%Y-%m-%d_%H.%M.%S");
    let elapsed_time = et - st;
    println!("Completion Time: {}", end_time);
    println!("Time Elapsed: {}", elapsed_time);
}
