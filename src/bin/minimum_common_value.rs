struct Solution;

impl Solution {
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
}

fn main() {}

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
}
