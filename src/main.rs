use std::io;
mod ui;
use ui::page_details::PageDetails;
 
fn main() {
    new();
}

fn new(){
    let json_data = PageDetails::read_json();
    PageDetails::new(&json_data);
}