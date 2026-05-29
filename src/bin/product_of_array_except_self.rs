/// 238. Product of Array Except Self
///
/// Primary approach: store prefix products in the output array, then multiply
/// each slot by the suffix product accumulated from the right.
/// Runtime: O(n)
/// Space: O(1) extra, excluding the output array
///
/// Other valid approaches:
/// - Explicit prefix and suffix arrays: O(n) runtime, O(n) extra space
/// - Brute force over all indices except self: O(n^2) runtime, O(1) extra
///   space
struct Solution;

impl Solution {
    /// Optimized two-pass solution using the output array for prefix products.
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![1; n];

        let mut prefix = 1;
        for i in 0..n {
            result[i] = prefix;
            prefix *= nums[i];
        }

        let mut suffix = 1;
        for i in (0..n).rev() {
            result[i] *= suffix;
            suffix *= nums[i];
        }

        result
    }

    /// Alternative solution with explicit prefix and suffix arrays.
    ///
    /// `left[i]` stores the product of all elements before `i`, and `right[i]`
    /// stores the product of all elements after `i`. Their product gives the
    /// answer for each position.
    pub fn product_except_self_prefix_suffix(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut left = vec![1; n];
        let mut right = vec![1; n];
        let mut result = vec![1; n];

        for i in 1..n {
            left[i] = left[i - 1] * nums[i - 1];
        }

        for i in (0..n - 1).rev() {
            right[i] = right[i + 1] * nums[i + 1];
        }

        for i in 0..n {
            result[i] = left[i] * right[i];
        }

        result
    }

    /// Brute-force solution.
    ///
    /// For each index, multiply every other element. This is easy to reason
    /// about but does not meet the required O(n) runtime bound.
    pub fn product_except_self_brute_force(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());

        for i in 0..nums.len() {
            let mut product = 1;

            for j in 0..nums.len() {
                if i != j {
                    product *= nums[j];
                }
            }

            result.push(product);
        }

        result
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4];

    println!("optimized: {:?}", Solution::product_except_self(nums.clone()));
    println!(
        "prefix/suffix: {:?}",
        Solution::product_except_self_prefix_suffix(nums.clone())
    );
    println!(
        "brute force: {:?}",
        Solution::product_except_self_brute_force(nums)
    );
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn solves_first_example() {
        assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
        assert_eq!(
            Solution::product_except_self_prefix_suffix(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self_brute_force(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
    }

    #[test]
    fn solves_second_example() {
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
        assert_eq!(
            Solution::product_except_self_prefix_suffix(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
        assert_eq!(
            Solution::product_except_self_brute_force(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }

    #[test]
    fn handles_single_zero() {
        assert_eq!(Solution::product_except_self(vec![0, 4, 5]), vec![20, 0, 0]);
        assert_eq!(
            Solution::product_except_self_prefix_suffix(vec![0, 4, 5]),
            vec![20, 0, 0]
        );
        assert_eq!(
            Solution::product_except_self_brute_force(vec![0, 4, 5]),
            vec![20, 0, 0]
        );
    }

    #[test]
    fn handles_two_elements() {
        assert_eq!(Solution::product_except_self(vec![2, 7]), vec![7, 2]);
        assert_eq!(
            Solution::product_except_self_prefix_suffix(vec![2, 7]),
            vec![7, 2]
        );
        assert_eq!(
            Solution::product_except_self_brute_force(vec![2, 7]),
            vec![7, 2]
        );
    }
}
