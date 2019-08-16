use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {data: String::from("my stuff")};

    let d = CustomSmartPointer {data: String::from("other stuff")};

    println!("CustomSmartPointers created.");

    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Lists::Cons(Rc::clone(&value), Rc::new(Lists::Nil)));
    let b = Lists::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Lists::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}!`", self.data);
    }
}

enum Listz {
    Cons(i32, Rc<Listz>),
    Nil,
}

#[derive(Debug)]
enum Lists {
    Cons(Rc<RefCell<i32>>, Rc<Lists>),
    Nil,
}