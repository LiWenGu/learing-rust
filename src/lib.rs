#[cfg(test)]
mod tests {
    use crate::Guess;

    #[test]
    #[should_panic(expected="asdwdawdawdwad")]
    fn greater_than_100() {
        Guess::new(5555);
    }
}

pub struct  Guess {
    value: i32,
}

impl Guess {
    pub fn new (value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be");
        }

        Guess {
            value
        }
    }
}