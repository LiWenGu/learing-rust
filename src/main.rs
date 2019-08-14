mod sound {
    pub mod instrument {
        pub fn clarinet() {
            super::breathe_in();
        }
    }

    fn breathe_in() {}
}

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str)->Vegetable{
            Vegetable{
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu {
    // 公共枚举属性都为公共
    pub enum Appetizer{
        Soup,
        Salad,
    }
}
// 绝对路径引入
use crate::sound::instrument;
// 相对路径引入
use self::sound::instrument;

fn main() {
    crate::sound::instrument::clarinet();

    sound::instrument::clarinet();

    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");

    println!("{} are delicious", v.name);
    // println!("{}", v.id);

    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;

    instrument::clarinet();
}