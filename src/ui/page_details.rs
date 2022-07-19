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
    pub index: i32
}
pub fn read_json() -> String {
    let json_data = fs::read_to_string("data.json").expect("Error reading Json file :(");
    json_data
}


impl PageDetails{
    pub fn new<'a>(json_str:&'a str) -> Page {
        let page = serde_json::from_str::<Page>(json_str)
        .expect("Error reading json to struct");
        println!("{:?}", page);
        page   
    }
    pub fn read_json() -> String {
        let json_data = fs::read_to_string("./data/data.json").expect("Error reading Json file :(");
        json_data
    }    
}
