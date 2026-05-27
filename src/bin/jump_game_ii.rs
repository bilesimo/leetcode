/// 45. Jump Game II
///
/// Primary approach: greedy level-by-level scan.
/// Runtime: O(n)
/// Space: O(1)
///
/// Other valid approaches:
/// - Dynamic programming over minimum jumps to each index: O(n^2) runtime,
///   O(n) space
struct Solution;

impl Solution {
    /// Greedy solution.
    ///
    /// Treat the array like BFS levels. `current_end` marks the end of the
    /// current jump range, and `farthest` records how far the next jump can
    /// extend that range. Every time we finish scanning the current range, we
    /// must spend one jump to move into the next range.
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() <= 1 {
            return 0;
        }

        let last = nums.len() - 1;
        let mut jumps = 0;
        let mut current_end = 0usize;
        let mut farthest = 0usize;

        for i in 0..last {
            farthest = farthest.max(i + nums[i] as usize);

            if i == current_end {
                jumps += 1;
                current_end = farthest;

                if current_end >= last {
                    break;
                }
            }
        }

        jumps
    }

    /// Dynamic programming solution.
    ///
    /// `dp[i]` stores the minimum number of jumps needed to reach index `i`.
    /// From each reachable index, try relaxing every position that can be
    /// reached in one more jump.
    pub fn jump_dp(nums: Vec<i32>) -> i32 {
        let last = nums.len() - 1;
        let mut dp = vec![i32::MAX; nums.len()];
        dp[0] = 0;

        for i in 0..nums.len() {
            let farthest = (i + nums[i] as usize).min(last);

            for next in i + 1..=farthest {
                dp[next] = dp[next].min(dp[i] + 1);
            }
        }

        dp[last]
    }
}

fn main() {
    let nums = vec![2, 3, 1, 1, 4];

    println!("greedy: {}", Solution::jump(nums.clone()));
    println!("dp: {}", Solution::jump_dp(nums));
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn greedy_returns_two_for_example_one() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn greedy_returns_two_for_example_two() {
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }

    #[test]
    fn dp_returns_two_for_example_one() {
        assert_eq!(Solution::jump_dp(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn dp_returns_two_for_example_two() {
        assert_eq!(Solution::jump_dp(vec![2, 3, 0, 1, 4]), 2);
    }

    #[test]
    fn both_solutions_handle_single_element_input() {
        assert_eq!(Solution::jump(vec![0]), 0);
        assert_eq!(Solution::jump_dp(vec![0]), 0);
    }

    #[test]
    fn both_solutions_handle_direct_jump_to_end() {
        assert_eq!(Solution::jump(vec![4, 1, 1, 1, 1]), 1);
        assert_eq!(Solution::jump_dp(vec![4, 1, 1, 1, 1]), 1);
    }

    #[test]
    fn both_solutions_handle_progressive_ranges() {
        assert_eq!(Solution::jump(vec![1, 2, 1, 1, 1]), 3);
        assert_eq!(Solution::jump_dp(vec![1, 2, 1, 1, 1]), 3);
    }
}
