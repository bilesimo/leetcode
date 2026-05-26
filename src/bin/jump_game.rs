/// 55. Jump Game
///
/// Primary approach: greedy scan tracking the farthest reachable index.
/// Runtime: O(n)
/// Space: O(1)
///
/// Other valid approaches:
/// - Dynamic programming over reachable indices: O(n^2) runtime, O(n) space
struct Solution;

impl Solution {
    /// Greedy solution.
    ///
    /// `farthest` stores the best reach available after scanning indices up to
    /// the current position. If we ever reach an index beyond `farthest`, that
    /// position is unreachable and the answer is false.
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut farthest = 0usize;
        let last = nums.len() - 1;

        for (i, &jump) in nums.iter().enumerate() {
            if i > farthest {
                return false;
            }

            farthest = farthest.max(i + jump as usize);

            if farthest >= last {
                return true;
            }
        }

        true
    }

    /// Dynamic programming solution.
    ///
    /// `reachable[i]` is true if there exists some sequence of jumps that lands
    /// on index `i`. From each reachable index, mark every forward position
    /// that can be reached in one more jump.
    pub fn can_jump_dp(nums: Vec<i32>) -> bool {
        let last = nums.len() - 1;
        let mut reachable = vec![false; nums.len()];
        reachable[0] = true;

        for i in 0..nums.len() {
            if !reachable[i] {
                continue;
            }

            let farthest = (i + nums[i] as usize).min(last);
            for next in i + 1..=farthest {
                reachable[next] = true;
            }

            if reachable[last] {
                return true;
            }
        }

        reachable[last]
    }
}

fn main() {
    let nums = vec![2, 3, 1, 1, 4];

    println!("greedy: {}", Solution::can_jump(nums.clone()));
    println!("dp: {}", Solution::can_jump_dp(nums));
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn greedy_returns_true_for_example_one() {
        assert!(Solution::can_jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn greedy_returns_false_for_example_two() {
        assert!(!Solution::can_jump(vec![3, 2, 1, 0, 4]));
    }

    #[test]
    fn dp_returns_true_for_example_one() {
        assert!(Solution::can_jump_dp(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn dp_returns_false_for_example_two() {
        assert!(!Solution::can_jump_dp(vec![3, 2, 1, 0, 4]));
    }

    #[test]
    fn both_solutions_handle_single_element_input() {
        assert!(Solution::can_jump(vec![0]));
        assert!(Solution::can_jump_dp(vec![0]));
    }

    #[test]
    fn both_solutions_handle_zero_after_reachable_jump() {
        assert!(Solution::can_jump(vec![2, 0, 0]));
        assert!(Solution::can_jump_dp(vec![2, 0, 0]));
    }

    #[test]
    fn both_solutions_detect_unreachable_gap() {
        assert!(!Solution::can_jump(vec![1, 0, 1, 0]));
        assert!(!Solution::can_jump_dp(vec![1, 0, 1, 0]));
    }
}
