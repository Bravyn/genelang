use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Page{
    pages: Vec<PageDetails>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PageDetails{
    pub title: String,
    pub link: bool,
    pub path: String,
    pub content: String,
    pub template: bool,
    pub index: i32,
}


/*impl PageDetails{
    fn new(title: String) -> PageDetails {
        PageDetails{
            title
        }
    }
}*/