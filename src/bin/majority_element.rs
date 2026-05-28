/// 169. Majority Element
///
/// Primary approach: Boyer-Moore voting.
/// Runtime: O(n)
/// Space: O(1)
///
/// Other valid approaches:
/// - HashMap counting: O(n) runtime, O(n) extra space
struct Solution;

impl Solution {
    /// Boyer-Moore voting algorithm.
    ///
    /// Pairs of different values cancel each other out. Because the majority
    /// element appears more than n / 2 times, it survives all cancellations
    /// and remains as the final candidate.
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = 0;
        let mut count = 0;

        for num in nums {
            if count == 0 {
                candidate = num;
            }

            if num == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }

        candidate
    }

    /// Alternative solution using a hash map to count occurrences.
    ///
    /// Runtime: O(n)
    /// Space: O(n)
    pub fn majority_element_hash_map(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let limit = nums.len() / 2;
        let mut counts = HashMap::new();

        for num in nums {
            let count = counts.entry(num).or_insert(0);
            *count += 1;

            if *count > limit {
                return num;
            }
        }

        unreachable!()
    }
}

fn main() {
    let nums = vec![2, 2, 1, 1, 1, 2, 2];

    println!(
        "Boyer-Moore Solution: {}",
        Solution::majority_element(nums.clone())
    );
    println!(
        "HashMap Solution: {}",
        Solution::majority_element_hash_map(nums)
    );
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn boyer_moore_returns_majority_in_small_example() {
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn boyer_moore_returns_majority_in_longer_example() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn hash_map_returns_majority_in_small_example() {
        assert_eq!(Solution::majority_element_hash_map(vec![3, 2, 3]), 3);
    }

    #[test]
    fn hash_map_returns_majority_in_longer_example() {
        assert_eq!(
            Solution::majority_element_hash_map(vec![2, 2, 1, 1, 1, 2, 2]),
            2
        );
    }

    #[test]
    fn both_solutions_handle_single_element_input() {
        assert_eq!(Solution::majority_element(vec![7]), 7);
        assert_eq!(Solution::majority_element_hash_map(vec![7]), 7);
    }
}
