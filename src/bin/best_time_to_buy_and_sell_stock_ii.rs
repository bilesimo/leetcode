/// 122. Best Time to Buy and Sell Stock II
///
/// Primary approach: sum every positive day-to-day gain.
/// Runtime: O(n)
/// Space: O(1)
struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut total = 0;

        for i in 1..prices.len() {
            let diff = prices[i] - prices[i - 1];

            if diff > 0 {
                total += diff;
            }
        }

        total
    }
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    println!("{}", Solution::max_profit(prices));
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn returns_profit_for_example_one() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    }

    #[test]
    fn returns_profit_for_example_two() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn returns_zero_when_prices_only_fall() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn handles_single_day_input() {
        assert_eq!(Solution::max_profit(vec![5]), 0);
    }

    #[test]
    fn handles_flat_segments_between_gains() {
        assert_eq!(Solution::max_profit(vec![1, 2, 2, 3]), 2);
    }
}
