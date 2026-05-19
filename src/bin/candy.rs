/// 135. Candy
///
/// Primary approach: two passes with a shared allocation array.
/// Runtime: O(n)
/// Space: O(n)
struct Solution;

impl Solution {
    /// First satisfy the left-neighbor constraint, then satisfy the
    /// right-neighbor constraint without undoing the first pass.
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut allocation = vec![1; n];

        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                allocation[i] = allocation[i - 1] + 1;
            }
        }

        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                allocation[i] = allocation[i].max(allocation[i + 1] + 1);
            }
        }

        allocation.iter().sum()
    }
}

fn main() {
    let result = Solution::candy(vec![1, 0, 2]);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn solves_first_example() {
        assert_eq!(Solution::candy(vec![1, 0, 2]), 5);
    }

    #[test]
    fn solves_second_example() {
        assert_eq!(Solution::candy(vec![1, 2, 2]), 4);
    }

    #[test]
    fn handles_single_child() {
        assert_eq!(Solution::candy(vec![7]), 1);
    }

    #[test]
    fn handles_strictly_increasing_ratings() {
        assert_eq!(Solution::candy(vec![1, 2, 3, 4]), 10);
    }

    #[test]
    fn handles_strictly_decreasing_ratings() {
        assert_eq!(Solution::candy(vec![4, 3, 2, 1]), 10);
    }
}
