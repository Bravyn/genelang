use std::io;
mod ui;
use ui::page_details::Page;
 
fn main() {
    let json_data = Page::read_json();//convert json data to page type
   
    let page_details = Page::new(&json_data);
    
    println!("{:?}", page_details.pages[0])
    
}

