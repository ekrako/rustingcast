#[derive(Debug)]
enum List {
    Data(i32, Box<List>),
    Nil,
}
use crate::List::{Data, Nil};

fn main() {
   let list = Data(1, Box::new(Data(2, Box::new(Data(3, Box::new(Nil))))));
   let box_list = Box::new(Data(1, Box::new(Data(2, Box::new(Data(3, Box::new(Nil)))))));
   println!("list:      {:?}", list);
   println!("box list:  {:?}", box_list);
   let x = 5;
   let y = &x;

   assert_eq!(5, x);
   assert_eq!(&5, y);
   assert_eq!(5, *y);
   let y = MyBox::new(x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref())); // deref is called automatically
}

use std::ops::Deref;

struct MyBox<T>(T);
impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}