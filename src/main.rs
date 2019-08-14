fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}, {}, {}", x, y, z);

    let a = [1, 2, 3, 4, 5];

    println!("{}", a[1]);

    let condition = true;

    let number = if condition {
        5
    } else {
        6
    };

    println!("{}", number);


    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s2);

    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}