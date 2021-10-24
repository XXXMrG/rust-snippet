#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // assert_eq! and assert_nq! 底层使用 == 和 !=
        // 因此被比较的值必须实现了 PartialEq 和 Debug trait
        // 通常可以直接在结构体或枚举上添加 #[derive(PartialEq, Debug)] 注解
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn it_fail() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7};
        let smaller = Rectangle { width: 5, height: 1};

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    #[should_panic]
    fn should_panic_fn() {
        println!("xxx");
        panic!("xxxx");
        // ppp::should_parivate();
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        // return
        self.width > other.width && self.height > other.height
    }

    pub fn get_area(&self) -> u32 {
        self.width * self.height
    }
}

mod ppp {
    fn should_parivate() {
        println!("ppp");
    }
}
