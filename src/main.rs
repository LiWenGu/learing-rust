use std::fmt::{Display, Debug};

fn main() {
    let number_list = vec![34, 50, 25, 100];
    let result = largest(&number_list);
    println!("{}", result);
    let number_list = vec![102, 42, 6000];
    let result = largest(&number_list);
    println!("{}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("T{}", result);

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3{},{}", p3.x, p3.y);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years age...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt{part : first_sentence};
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn some_function<T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    2
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}