/// 274. H-Index
///
/// Primary approach: bucket counting.
/// Runtime: O(n)
/// Space: O(n)
struct Solution;

impl Solution {
    /// Count how many papers have each citation count, clamping values above
    /// `n` into the last bucket because h-index can never exceed the number of
    /// papers.
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut buckets = vec![0; n + 1];

        for citation in citations {
            let citation = citation.max(0) as usize;

            if citation >= n {
                buckets[n] += 1;
            } else {
                buckets[citation] += 1;
            }
        }

        let mut papers = 0;

        for h in (0..=n).rev() {
            papers += buckets[h];

            if papers >= h {
                return h as i32;
            }
        }

        0
    }
}

fn main() {
    let result = Solution::h_index(vec![3, 0, 6, 1, 5]);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn solves_first_example() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    }

    #[test]
    fn solves_second_example() {
        assert_eq!(Solution::h_index(vec![1, 3, 1]), 1);
    }

    #[test]
    fn handles_all_zero_citations() {
        assert_eq!(Solution::h_index(vec![0, 0, 0]), 0);
    }

    #[test]
    fn handles_all_high_citations() {
        assert_eq!(Solution::h_index(vec![100, 100, 100, 100]), 4);
    }

    #[test]
    fn handles_mixed_counts_near_threshold() {
        assert_eq!(Solution::h_index(vec![4, 4, 0, 0]), 2);
    }
}
