use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct SiteData {
    pub title: String,
    pub link: String,
    pub domain_name: String, 
}