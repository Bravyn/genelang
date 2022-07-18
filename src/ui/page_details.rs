pub struct PageDetails{
    pub title: String
}
impl PageDetails{
    fn new(title: String) -> PageDetails {
        PageDetails{
            title
        }
    }
}