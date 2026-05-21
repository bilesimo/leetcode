/// 80. Remove Duplicates from Sorted Array II
///
/// Primary approach: keep a write pointer and allow a value only when it does
/// not match the value two slots behind the compacted prefix.
/// Runtime: O(n)
/// Space: O(1)
struct Solution;

impl Solution {
    /// The first two values are always allowed. After that, a value can be
    /// kept only if it differs from the value at `write - 2`.
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();

        if len < 3 {
            return len as i32;
        }

        let mut write = 2;

        for read in 2..len {
            if nums[read] != nums[write - 2] {
                nums[write] = nums[read];
                write += 1;
            }
        }

        write as i32
    }
}

fn main() {
    let mut nums = vec![1, 1, 1, 2, 2, 3];
    let len = Solution::remove_duplicates(&mut nums);

    println!("len={len}, nums={:?}", &nums[..len as usize]);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn removes_extra_duplicates_in_first_example() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];

        let len = Solution::remove_duplicates(&mut nums);

        assert_eq!(len, 5);
        assert_eq!(&nums[..len as usize], &[1, 1, 2, 2, 3]);
    }

    #[test]
    fn removes_extra_duplicates_in_second_example() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];

        let len = Solution::remove_duplicates(&mut nums);

        assert_eq!(len, 7);
        assert_eq!(&nums[..len as usize], &[0, 0, 1, 1, 2, 3, 3]);
    }

    #[test]
    fn handles_all_values_already_within_limit() {
        let mut nums = vec![1, 1, 2, 2, 3, 3];

        let len = Solution::remove_duplicates(&mut nums);

        assert_eq!(len, 6);
        assert_eq!(&nums[..len as usize], &[1, 1, 2, 2, 3, 3]);
    }

    #[test]
    fn handles_single_repeated_value() {
        let mut nums = vec![4, 4, 4, 4, 4];

        let len = Solution::remove_duplicates(&mut nums);

        assert_eq!(len, 2);
        assert_eq!(&nums[..len as usize], &[4, 4]);
    }

    #[test]
    fn handles_short_inputs() {
        let mut nums = vec![2, 2];

        let len = Solution::remove_duplicates(&mut nums);

        assert_eq!(len, 2);
        assert_eq!(&nums[..len as usize], &[2, 2]);
    }
}
