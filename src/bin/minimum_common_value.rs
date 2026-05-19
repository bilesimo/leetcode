/// 2540. Minimum Common Value
///
/// Primary approach: two pointers.
/// Runtime: O(n + m)
/// Space: O(1)
///
/// Other valid approaches:
/// - Hash set: O(n + m) runtime, O(n) extra space
/// - Binary search each element of the smaller array in the larger one:
///   O(min(n, m) * log(max(n, m))) runtime, O(1) extra space
struct Solution;

impl Solution {
    /// Two-pointer solution.
    ///
    /// Because both input arrays are sorted, we can discard one value on each
    /// step by advancing the pointer that currently points to the smaller
    /// number. The first match is the minimum common value.
    pub fn get_common(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 0;

        // Both arrays are sorted, so we can walk them from left to right.
        // Always advance the pointer that points to the smaller value.
        while left < nums1.len() && right < nums2.len() {
            match nums1[left].cmp(&nums2[right]) {
                // The first equal value we find is the minimum common value.
                std::cmp::Ordering::Equal => return nums1[left],
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right += 1,
            }
        }

        // We exhausted one array without finding a match.
        -1
    }

    /// Alternative solution using a hash set.
    ///
    /// Runtime: O(n + m)
    /// Space: O(n)
    pub fn get_common_hash_set(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let seen: HashSet<i32> = nums1.into_iter().collect();

        for num in nums2 {
            if seen.contains(&num) {
                return num;
            }
        }

        -1
    }
}

fn main() {
    println!(
        "Solution: {}",
        Solution::get_common(vec![1, 2, 3], vec![2, 4])
    );
    println!(
        "Hash Set Solution: {}",
        Solution::get_common_hash_set(vec![1, 2, 3], vec![2, 4])
    );
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn returns_minimum_common_value() {
        assert_eq!(Solution::get_common(vec![1, 2, 3], vec![2, 4]), 2);
    }

    #[test]
    fn returns_negative_one_when_no_common_value_exists() {
        assert_eq!(Solution::get_common(vec![1, 2, 3], vec![4, 5, 6]), -1);
    }

    #[test]
    fn hash_set_solution_returns_minimum_common_value() {
        assert_eq!(Solution::get_common_hash_set(vec![1, 2, 3], vec![2, 4]), 2);
    }

    #[test]
    fn hash_set_solution_returns_negative_one_when_no_common_value_exists() {
        assert_eq!(
            Solution::get_common_hash_set(vec![1, 2, 3], vec![4, 5, 6]),
            -1
        );
    }
}
