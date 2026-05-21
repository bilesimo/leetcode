/// 26. Remove Duplicates from Sorted Array
///
/// Primary approach: keep a write pointer for the next unique slot.
/// Runtime: O(n)
/// Space: O(1)
struct Solution;

impl Solution {
    /// Since the input is sorted, each new unique value differs from the value
    /// just written to the compacted prefix.
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut write = 1;

        for read in 1..nums.len() {
            if nums[read] != nums[write - 1] {
                nums[write] = nums[read];
                write += 1;
            }
        }

        write as i32
    }
}

fn main() {
    let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    let len = Solution::remove_duplicates(&mut nums);

    println!("len={len}, nums={:?}", &nums[..len as usize]);
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn removes_duplicates_in_small_example() {
        let mut nums = vec![1, 1, 2];

        let len = Solution::remove_duplicates(&mut nums);

        assert_eq!(len, 2);
        assert_eq!(&nums[..len as usize], &[1, 2]);
    }

    #[test]
    fn removes_duplicates_in_longer_example() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];

        let len = Solution::remove_duplicates(&mut nums);

        assert_eq!(len, 5);
        assert_eq!(&nums[..len as usize], &[0, 1, 2, 3, 4]);
    }

    #[test]
    fn handles_all_unique_values() {
        let mut nums = vec![1, 2, 3, 4];

        let len = Solution::remove_duplicates(&mut nums);

        assert_eq!(len, 4);
        assert_eq!(&nums[..len as usize], &[1, 2, 3, 4]);
    }

    #[test]
    fn handles_all_duplicate_values() {
        let mut nums = vec![7, 7, 7, 7];

        let len = Solution::remove_duplicates(&mut nums);

        assert_eq!(len, 1);
        assert_eq!(&nums[..len as usize], &[7]);
    }
}
