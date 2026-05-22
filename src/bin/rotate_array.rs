/// 189. Rotate Array
///
/// Primary approach: reverse the whole array, then reverse the first `k`
/// elements and the remaining suffix.
/// Runtime: O(n)
/// Space: O(1)
struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        if len <= 1 {
            return;
        }

        let k = k as usize % len;
        if k == 0 {
            return;
        }

        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}

fn main() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(&mut nums, 3);

    println!("{nums:?}");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn rotates_example_one() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];

        Solution::rotate(&mut nums, 3);

        assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn rotates_example_two() {
        let mut nums = vec![-1, -100, 3, 99];

        Solution::rotate(&mut nums, 2);

        assert_eq!(nums, vec![3, 99, -1, -100]);
    }

    #[test]
    fn handles_k_larger_than_length() {
        let mut nums = vec![1, 2];

        Solution::rotate(&mut nums, 3);

        assert_eq!(nums, vec![2, 1]);
    }

    #[test]
    fn handles_zero_rotation() {
        let mut nums = vec![1, 2, 3];

        Solution::rotate(&mut nums, 0);

        assert_eq!(nums, vec![1, 2, 3]);
    }

    #[test]
    fn handles_single_element() {
        let mut nums = vec![42];

        Solution::rotate(&mut nums, 10);

        assert_eq!(nums, vec![42]);
    }
}
