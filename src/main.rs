use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::new();

    let v= vec![1,2,3,4];
    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    };

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);


    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    for (key, value) in &scores {
        println!("{}, {}", key, value);
    }
    let key = String::from("Yello");
    scores.entry(&key).or_insert(&50);
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}