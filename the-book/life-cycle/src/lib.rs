#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
// 生命周期注解保证了参数的引用和返回值的生命周期一致，返回值的借用始终是合法的
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_no_lifecycle(x: String, y: String) -> String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn no_lifecycle_run() {
    let string1 = String::from("long string is long ");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest_no_lifecycle(string1, string2);
        println!("{}", result);
    }

    // no borrow, so rusult always have life
    println!("{}", result);
}

pub fn run() {
    let string1 = String::from("long string is long ");
    let result;

    {
        let string2 = String::from("xyz");
        // let string2: &'static str = "xyz";
        result = longest(string1.as_str(), string2.as_str());
        println!("{}", result);
    }

    // can't use result, cause string2 is dead
    // println!("{}", result);
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

pub fn new_excerpt() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let i;

    {
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");

        i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    // novel not dead, so i.part not dead, even first_sentence dead
    println!("{}", i.part);
}

use std::fmt::Display;

pub fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
