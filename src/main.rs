fn main() {
    let m = Representation::new(vec!["hello"], vec!["hey"]);
    println!("{:?}", m);
}

#[derive(Debug)]
struct Representation<T>{
    parent: Vec<T>,
    child: Vec<T>,
}

impl<T> Representation<T>{
    fn new(parent:Vec<T>, child:Vec<T>) -> Representation<T> {
        Representation {
            parent,child
         }
    }
}
