use chrono;
//use regex::Regex;
use reqwest::StatusCode;
use scraper:: { Html, Selector };
use std::fs::File;
use std::io::Write;

mod models;
mod utils;

#[tokio::main]
async fn main() {
    let st = chrono::Local::now();
    let start_time = st.format("%Y-%m-%d_%H.%M.%S");
    println!("Start Time: {}", start_time);
    let client = utils::get_client();
    let domain_names: [String; 4] = ["bitcoin.org".to_string(), "y.cash".to_string(), "z.cash".to_string(), "verus.io".to_string()];
    for domain_name in &domain_names {
        let filename = format!("{}.csv", domain_name);
        let mut writer = File::create(&filename).unwrap();
        let url = format!("https://{}", &domain_name);
        let result = client.get(url).send().await.unwrap();
        let dt = chrono::Local::now();
        println!("{}", dt.format("%Y-%m-%d_%H.%M.%S"));
        let raw_html = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("Something went wrong!"),
        };

        let mut site_data: Vec<models::Site_Data> = Vec::new();

        let document = Html::parse_document(&raw_html);
        let selector = Selector::parse("a").unwrap();

        for element in document.select(&selector) {
            let inner = element.inner_html().to_string();
            let href = match element.value().attr("href") {
            Some(target_url) => target_url,
            _ => "no url found"
            };
        
    println!("Title: {}", &inner);
    println!("Link: {}", href);
    writeln!(
        &mut writer,
        "{},{}",
        &serde_json::to_string(&inner).unwrap(),
        &serde_json::to_string(&href).unwrap());
        }
    //save_article_list(&site_data, domain_name);
    let et = chrono::Local::now();
    let end_time = et.format("%Y-%m-%d_%H.%M.%S");
    let elapsed_time = et - st;
    println!("Completion Time: {}", end_time);
    println!("Time Elapsed: {}", elapsed_time);

    }
}


fn save_article_list(site_data: &Vec<models::Site_Data>, domain_name: &str) {
    let dt = chrono::Local::now();
    let filename = format!("{}_{}.csv", domain_name, dt.format("%Y-%m-%d_%H.%M.%S"));
    let mut writer = File::create(&filename).unwrap();
    for site_data in site_data.iter() {
    writeln!(
        &mut writer,
        "{}",
        &serde_json::to_string(&site_data).unwrap(),
    ).unwrap();
    }
}
