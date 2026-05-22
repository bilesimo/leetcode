/// 121. Best Time to Buy and Sell Stock
///
/// Primary approach: track the lowest price seen so far and the best profit
/// achievable by selling on each day.
/// Runtime: O(n)
/// Space: O(1)
///
/// Other valid approaches:
/// - Brute force over all buy/sell pairs: O(n^2) runtime, O(1) extra space
struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = prices[0];
        let mut max_profit = 0;

        for &price in prices.iter().skip(1) {
            max_profit = max_profit.max(price - min_price);
            min_price = min_price.min(price);
        }

        max_profit
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
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
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
    fn handles_best_trade_at_the_end() {
        assert_eq!(Solution::max_profit(vec![9, 2, 1, 7]), 6);
    }

    #[test]
    fn handles_repeated_prices() {
        assert_eq!(Solution::max_profit(vec![3, 3, 3, 3]), 0);
    }
}
