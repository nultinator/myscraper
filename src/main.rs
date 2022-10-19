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
    //Get domain name info
    println!("Enter a domain name:");
    let mut domain = String::new();
    std::io::stdin().read_line(&mut domain).expect("Please enter a valid domain");
    println!("You entered {}", domain);

    //Choose an element to scrape
    println!("What would you like to scrape?");
    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Please enter a valid choice");

    //Start timer
    let st = chrono::Local::now();
    let start_time = st.format("%Y-%m-%d_%H.%M.%S");
    println!("Start Time: {}", start_time);

    //Retrieve website
    let client = utils::get_client();
    let filename = format!("{}-{}.csv", domain.trim(), choice.trim());
    let mut writer = File::create(&filename).unwrap();
    let url = format!("https://{}", &domain);
    let result = client.get(url).send().await.unwrap();

    //Timestamp completion
    let dt = chrono::Local::now();
    println!("{}", dt.format("%Y-%m-%d_%H.%M.%S"));
    let raw_html = match result.status() {
    StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("Something went wrong!"),
        };


    //Parse site data
    let site_data: Vec<models::SiteData> = Vec::new();

    let document = Html::parse_document(&raw_html);
    let selector = Selector::parse(&choice).unwrap();

    
    //Print to console and write selected data to a file
    for element in document.select(&selector) {
        let inner = element.inner_html().to_string();
        let href = match element.value().attr("href") {
            Some(target_url) => target_url,
            _ => "no url found"
        };

        println!("{}", element.inner_html().to_string());
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
