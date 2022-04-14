/**
 * [673] Number of Longest Increasing Subsequence
 *
 * Given an integer array nums, return the number of longest increasing subsequences.
Notice that the sequence has to be strictly increasing.

Example 1:
Input: nums = [1,3,5,4,7]
Output: 2
Explanation: The two longest increasing subsequences are [1, 3, 4, 7] and [1, 3, 5, 7].

Example 2:
Input: nums = [2,2,2,2,2]
Output: 5
Explanation: The length of longest continuous increasing subsequence is 1, and there are 5 subsequences' length is 1, so output 5.


Constraints:

    1 <= nums.length <= 2000
    -106 <= nums[i] <= 106

 */
pub struct Solution {}

// submission codes start here

impl Solution {
    // dp[i]: max lis len at a[i]
    // dp[0] = 1
    // dp[i] = max of dp[j] for j in 0..i + (if a[j] > a[i] => 1
    //                                       else 0)
    // pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
    //     let n = nums.len();
    //     if n == 1 { return 1; }

    //     let mut dp = vec![1; n];
    //     let mut cnt = vec![1; n];
    //     let mut len = 1;
    //     for i in 1..n {
    //         for j in 0..i {
    //             if nums[j] < nums[i] {
    //                 if dp[i] == dp[j] + 1 {
    //                     cnt[i] += cnt[j];
    //                 } else if dp[i] < dp[j] + 1 {
    //                     dp[i] = dp[j] + 1;
    //                     cnt[i] = cnt[j];
    //                 }
    //             }
    //         }
    //         len = len.max(dp[i]);
    //     }
    //     (0..n).filter(|&i| dp[i] == len).map(|i| cnt[i]).sum()
    // }

    // patience sort
    // binary_search != lower/upper_bound!
    pub fn find_number_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 1;
        }
        let mut deck = vec![vec![(i32::MAX, 1)]; n];
        let mut len = 0;
        for x in nums {
            let pos = deck
                // here use lower_bound
                .binary_search_by_key(&x, |d: &Vec<(i32, i32)>| d[0].0)
                .unwrap_or_else(|e| e);
            if pos == 0 {
                deck[0].insert(0, (x, 1));
            } else {
                let prev_deck = &deck[pos - 1];
                // find all y in prev_deck < x
                let mut i = prev_deck
                    // here use lower_bound
                    .binary_search_by_key(&x, |y| y.0)
                    .unwrap_or_else(|e| e);
                // at least x > prev_deck's top
                assert!(i > 0);
                if prev_deck[i].0 >= x { i -= 1;}
                let take = (i + 1).min(prev_deck.len() - 1); // last MAX not take in
                let prev_sum = prev_deck[0..take].iter().map(|(_, cnt)| cnt).sum();
                deck[pos].insert(0, (x, prev_sum))
            }
            if pos == len {
                len += 1;
            }
        }
        deck[len - 1].iter().rev().skip(1).map(|(_, cnt)| cnt).sum()   
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(1, Solution::find_number_of_lis(vec![100,90,80,70,60,50,60,70,80,90,100]));
        assert_eq!(2, Solution::find_number_of_lis(vec![1,3,5,4,7]));
        assert_eq!(5, Solution::find_number_of_lis(vec![2,2,2,2,2]));
        assert_eq!(3, Solution::find_number_of_lis(vec![1,2,4,3,5,4,7,2]));
    }
}
