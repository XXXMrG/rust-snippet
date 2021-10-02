pub struct Breakfast {
    pub toast: String,

    // still private
    seasonal_fruit: String,
}

impl Breakfast {
    // cause have a private member, need a constructor
    // caller can change toast, but can't change private member
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peach")
        }
    }

    pub fn log(&self) -> &String{
        // brow it
        &self.seasonal_fruit
    }
}

// enum member default are pub
pub enum Appetizer {
    Soup,
    Salad,
}