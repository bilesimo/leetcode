//! Shared helpers for LeetCode solutions.
//!
//! Keep common data structures and utilities here when multiple problems need
//! them.

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut seen = HashMap::new();

    for (index, value) in nums.iter().enumerate() {
        let complement = target - value;

        if let Some(previous_index) = seen.get(&complement) {
            return vec![*previous_index as i32, index as i32];
        }

        seen.insert(*value, index);
    }

    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn solves_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
