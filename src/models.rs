use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Site_Data {
    pub title: String,
    pub link: String,
    pub domain_name: String, 
}