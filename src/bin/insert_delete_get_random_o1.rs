use rand::Rng;
use std::collections::HashMap;

/// 380. Insert Delete GetRandom O(1)
///
/// Primary approach: hash map plus dense vector.
/// Average runtime per operation: O(1)
/// Space: O(n)
struct RandomizedSet {
    map: HashMap<i32, usize>,
    values: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            values: vec![],
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }

        self.values.push(val);
        self.map.insert(val, self.values.len() - 1);
        true
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(rem_index) = self.map.get(&val).copied() {
            self.map.remove(&val);

            if rem_index != self.values.len() - 1 {
                let target_index = self.values.len() - 1;
                self.values.swap(rem_index, target_index);
                self.map.insert(self.values[rem_index], rem_index);
            }

            self.values.pop();
            return true;
        }

        false
    }

    fn get_random(&self) -> i32 {
        if self.values.is_empty() {
            return 0;
        }

        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.values.len());
        self.values[index]
    }
}

fn main() {
    let mut randomized_set = RandomizedSet::new();

    println!("{}", randomized_set.insert(1));
    println!("{}", randomized_set.remove(2));
    println!("{}", randomized_set.insert(2));
    println!("{}", randomized_set.get_random());
    println!("{}", randomized_set.remove(1));
    println!("{}", randomized_set.insert(2));
    println!("{}", randomized_set.get_random());
}

#[cfg(test)]
mod tests {
    use super::RandomizedSet;

    #[test]
    fn supports_example_sequence() {
        let mut set = RandomizedSet::new();

        assert!(set.insert(1));
        assert!(!set.remove(2));
        assert!(set.insert(2));

        let value = set.get_random();
        assert!(value == 1 || value == 2);

        assert!(set.remove(1));
        assert!(!set.insert(2));
        assert_eq!(set.get_random(), 2);
    }

    #[test]
    fn rejects_duplicate_inserts() {
        let mut set = RandomizedSet::new();

        assert!(set.insert(7));
        assert!(!set.insert(7));
    }

    #[test]
    fn removes_swapped_middle_element_correctly() {
        let mut set = RandomizedSet::new();

        assert!(set.insert(10));
        assert!(set.insert(20));
        assert!(set.insert(30));
        assert!(set.remove(20));
        assert!(!set.remove(20));

        let value = set.get_random();
        assert!(value == 10 || value == 30);
    }
}
