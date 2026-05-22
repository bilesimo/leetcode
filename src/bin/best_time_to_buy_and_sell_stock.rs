/// 121. Best Time to Buy and Sell Stock
///
/// Primary approach: track the lowest price seen so far and the best profit
/// achievable by selling on each day.
/// Runtime: O(n)
/// Space: O(1)
///
/// Other valid approaches:
/// - Kadane's algorithm over day-to-day price differences: O(n) runtime,
///   O(1) extra space
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

    /// Alternative solution using Kadane's algorithm.
    ///
    /// Instead of thinking directly in terms of buy and sell days, convert the
    /// input into day-to-day gains and losses. For example:
    ///
    /// prices = [7, 1, 5, 3, 6, 4]
    /// diffs  = [-6, 4, -2, 3, -2]
    ///
    /// Buying on day `l` and selling on day `r` gives:
    /// prices[r] - prices[l]
    ///
    /// That is exactly the sum of the contiguous diff slice from `l + 1` to
    /// `r`, so the problem becomes: find the maximum subarray sum, but never
    /// return a negative value because "do nothing" is allowed.
    pub fn max_profit_kadane(prices: Vec<i32>) -> i32 {
        let mut current = 0;
        let mut best = 0;

        for i in 1..prices.len() {
            let diff = prices[i] - prices[i - 1];

            // Either extend the current run of profitable day-to-day changes,
            // or reset to zero and effectively start a new buy day later.
            current = (current + diff).max(0);
            best = best.max(current);
        }

        best
    }
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];

    println!("min-price: {}", Solution::max_profit(prices.clone()));
    println!("kadane: {}", Solution::max_profit_kadane(prices));
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn returns_profit_for_example_one() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit_kadane(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn returns_zero_when_prices_only_fall() {
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit_kadane(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn handles_single_day_input() {
        assert_eq!(Solution::max_profit(vec![5]), 0);
        assert_eq!(Solution::max_profit_kadane(vec![5]), 0);
    }

    #[test]
    fn handles_best_trade_at_the_end() {
        assert_eq!(Solution::max_profit(vec![9, 2, 1, 7]), 6);
        assert_eq!(Solution::max_profit_kadane(vec![9, 2, 1, 7]), 6);
    }

    #[test]
    fn handles_repeated_prices() {
        assert_eq!(Solution::max_profit(vec![3, 3, 3, 3]), 0);
        assert_eq!(Solution::max_profit_kadane(vec![3, 3, 3, 3]), 0);
    }

    #[test]
    fn both_implementations_match_on_mixed_case() {
        let prices = vec![2, 4, 1, 8, 3, 9];

        assert_eq!(Solution::max_profit(prices.clone()), 8);
        assert_eq!(Solution::max_profit_kadane(prices), 8);
    }
}
