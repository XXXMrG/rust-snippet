use std::collections::HashMap;
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cacher_tester() {
        let mut count = 0;
        let mut cacher_result = Cacher::new(|num| {
            thread::sleep(Duration::from_secs(2));
            count += 1;
            num
        });

        assert_eq!(cacher_result.value(2), 2);
        assert_eq!(cacher_result.value(3), 3);
        assert_eq!(cacher_result.value(2), 2);

        assert_eq!(count, 2);
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();

        assert_eq!(sum, 18);
    }
}

/// # Cacher
///
/// ## Examples
/// ```
///
/// use functional::Cacher;
///
/// let mut count = 0;
/// let mut cacher_result = Cacher::new(|num| {
///     count += 1;
///     num
/// });
///
/// assert_eq!(cacher_result.value(2), 2);
/// assert_eq!(cacher_result.value(2), 2);
///
/// assert_eq!(count, 1);
/// ```
pub struct Cacher<T>
where
    T: FnMut(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: FnMut(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        match self.value.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
                v
            }
        }
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
