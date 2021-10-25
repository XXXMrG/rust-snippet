mod back_of_house;
/// ch.7 packages and crate
mod front_of_house;

// 重导出一个 mod 供外部调用
pub use crate::back_of_house::{Appetizer, Breakfast};
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let fast = Breakfast::summer("fa gun");
    println!("fruit is: {}", fast.log());

    Appetizer::Salad;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
