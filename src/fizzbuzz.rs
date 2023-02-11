pub struct FizzBuzz {
    number: i32,
}

impl FizzBuzz {
    pub fn new(number: i32) -> Self {
        Self { number }
    }

    pub fn fizzbuzz(&self) -> String {
        match (self.number % 3 == 0, self.number % 5 == 0) {
            (true, true) => "fizzbuzz".to_string(),
            (false, true) => "buzz".to_string(),
            (true, false) => "fizz".to_string(),
            _ => self.number.to_string(),
        }

        /*
        if self.number % 15 == 0 {
            "fizzbuzz".to_string()
        } else if self.number % 5 == 0 {
            "buzz".to_string()
        } else if self.number % 3 == 0 {
            "fizz".to_string()
        } else {
            self.number.to_string()
        }
        */
    }
}
