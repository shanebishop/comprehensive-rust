use std::collections::HashMap;
use std::hash::Hash;

/// Counter counts the number of times each value of type T has been seen.
struct Counter<T: Hash + Eq> {
    values: HashMap<T, u64>,
}

impl<T: Hash + Eq> Counter<T> {
    /// Create a new Counter.
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// Count an occurrence of the given value.
    fn count(&mut self, value: T) {
        // if self.values.contains_key(&value) {
        //     *self.values.get_mut(&value).unwrap() += 1;
        // } else {
        //     self.values.insert(value, 1);
        // }
        self.values
            .entry(value)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    /// Return the number of times the given value has been seen.
    fn times_seen(&self, value: T) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

fn main() {
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("saw {} values equal to {}", ctr.times_seen(i), i);
    }

    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    println!("got {} apples", strctr.times_seen("apple"));
}

#[test]
fn test_i32_counter() {
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);
    assert_eq!(ctr.times_seen(10), 0);
    assert_eq!(ctr.times_seen(11), 1);
    assert_eq!(ctr.times_seen(12), 0);
    assert_eq!(ctr.times_seen(13), 1);
    assert_eq!(ctr.times_seen(14), 3);
    assert_eq!(ctr.times_seen(15), 0);
    assert_eq!(ctr.times_seen(16), 1);
    assert_eq!(ctr.times_seen(17), 0);
    assert_eq!(ctr.times_seen(18), 0);
    assert_eq!(ctr.times_seen(19), 0);
}

#[test]
fn test_str_counter() {
    let mut strctr = Counter::new();
    strctr.count("apple");
    strctr.count("orange");
    strctr.count("apple");
    assert_eq!(strctr.times_seen("apple"), 2);
}
