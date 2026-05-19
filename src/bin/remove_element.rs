/// 27. Remove Element
///
/// Primary approach: overwrite kept values from left to right.
/// Runtime: O(n)
/// Space: O(1)
struct Solution;

impl Solution {
    /// Keep a write pointer for the next slot that should contain a value
    /// different from `val`.
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut write = 0;

        for read in 0..nums.len() {
            if nums[read] != val {
                nums[write] = nums[read];
                write += 1;
            }
        }

        write as i32
    }
}

fn main() {
    let mut nums = vec![3, 2, 2, 3];
    let len = Solution::remove_element(&mut nums, 3);

    println!("len={len}, nums={:?}", &nums[..len as usize]);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn removes_example_case() {
        let mut nums = vec![3, 2, 2, 3];

        let len = Solution::remove_element(&mut nums, 3);

        assert_eq!(len, 2);
        assert_eq!(&nums[..len as usize], &[2, 2]);
    }

    #[test]
    fn removes_multiple_occurrences() {
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];

        let len = Solution::remove_element(&mut nums, 2);

        assert_eq!(len, 5);
        assert_eq!(&nums[..len as usize], &[0, 1, 3, 0, 4]);
    }

    #[test]
    fn handles_no_matches() {
        let mut nums = vec![1, 2, 3];

        let len = Solution::remove_element(&mut nums, 4);

        assert_eq!(len, 3);
        assert_eq!(&nums[..len as usize], &[1, 2, 3]);
    }

    #[test]
    fn handles_all_matches() {
        let mut nums = vec![5, 5, 5];

        let len = Solution::remove_element(&mut nums, 5);

        assert_eq!(len, 0);
    }

    #[test]
    fn handles_empty_input() {
        let mut nums = vec![];

        let len = Solution::remove_element(&mut nums, 1);

        assert_eq!(len, 0);
    }
}
