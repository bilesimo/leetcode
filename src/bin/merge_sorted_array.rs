/// 88. Merge Sorted Array
///
/// Primary approach: fill `nums1` from the back.
/// Runtime: O(m + n)
/// Space: O(1)
struct Solution;

impl Solution {
    /// Walk both arrays from right to left and place the larger value into the
    /// current write position at the end of `nums1`.
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut write = (m + n) as usize;
        let mut left = m as usize;
        let mut right = n as usize;

        while right > 0 {
            write -= 1;

            if left > 0 && nums1[left - 1] > nums2[right - 1] {
                nums1[write] = nums1[left - 1];
                left -= 1;
            } else {
                nums1[write] = nums2[right - 1];
                right -= 1;
            }
        }
    }
}

fn main() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];

    Solution::merge(&mut nums1, 3, &mut nums2, 3);

    println!("{nums1:?}");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn merges_example_case() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];

        Solution::merge(&mut nums1, 3, &mut nums2, 3);

        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn handles_empty_second_array() {
        let mut nums1 = vec![1];
        let mut nums2 = vec![];

        Solution::merge(&mut nums1, 1, &mut nums2, 0);

        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn handles_empty_first_array_payload() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];

        Solution::merge(&mut nums1, 0, &mut nums2, 1);

        assert_eq!(nums1, vec![1]);
    }
}
