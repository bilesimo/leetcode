struct Solution;

impl Solution {
    pub fn solve(nums: Vec<i32>) -> i32 {
        nums.into_iter().sum()
    }
}

fn main() {
    let result = Solution::solve(vec![1, 2, 3]);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn solves_example_case() {
        assert_eq!(Solution::solve(vec![1, 2, 3]), 6);
    }
}
