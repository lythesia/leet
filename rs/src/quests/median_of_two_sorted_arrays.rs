/**
 * [4] Median of Two Sorted Arrays
 *
 * There are two sorted arrays nums1 and nums2 of size m and n respectively.

Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).

You may assume nums1 and nums2 cannot be both empty.

Example 1:

nums1 = [1, 3]
nums2 = [2]

The median is 2.0


Example 2:

nums1 = [1, 2]
nums2 = [3, 4]

The median is (2 + 3)/2 = 2.5


 */
pub struct Solution {}

// submission codes start here

use std::cmp;
impl Solution {
    fn find_kth(x: &[i32], y: &[i32], k: usize) -> f64 {
        if x.len() > y.len() {
            return Self::find_kth(y, x, k);
        }
        if x.is_empty() {
            y[k - 1] as f64
        } else if k == 1{
            cmp::min(x[0], y[0]) as f64
        } else {
            let k1 = cmp::min(k/2, x.len());
            let k2 = k - k1;
            if x[k1 - 1] == y[k2 - 1] {
                x[k1 - 1] as f64
            } else if x[k1 - 1] < y[k2 - 1] {
                Self::find_kth(&x[k1..], y, k2)
            } else {
                Self::find_kth(x, &y[k2..], k1)
            }
        }
    }

    fn kth(x: &[i32], y: &[i32], k: usize) -> f64 {
        // [x] <= [y]
        if x.len() > y.len() {
            Self::kth(y, x, k)
        // exit_1: one's empty
        } else if x.is_empty() {
            y[k - 1] as f64
        // exit_2: k >= 1 always, so k at least 1
        } else if k == 1 {
            std::cmp::min(x[0], y[0]) as f64
        } else {
            /*
            find kth in union(x, y)
            x: [x_1.. x_k/2.. xm]
            y: [y_1..    y_k/2 ..  ym]
            if x_k/2 < y_k/2
            then both x': [x_1 .. x_k/2] k/2 numbers of x < y_k/2
                 and y': [y_1 .. y_(k/2-1)] k/2-1 numbers of y < y_k/2
            even merge(x', y') has k-1 numbers which < kth of merge(x, y)
            so kth must be in merge of x'':[x_(k/2+1) .. xm] and y (because we cant tell <> between x'' and y')
            and should be (k - k/2)th of merge(x'', y)
            {}
            else
            the verse
            */
            let k1 = std::cmp::min(k/2, x.len()); 
            let k2 = k - k1;
            // exit_3: just bingo
            if x[k1 - 1] == y[k2 - 1] {
                x[k1 - 1] as f64
            } else if x[k1 - 1] < y[k2 - 1] {
                Self::kth(&x[k1..], y, k2)
            } else {
                Self::kth(x, &y[k2..], k1)
            }
        }
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n = nums1.len() + nums2.len(); // n >= 2
        let (x, y) = (nums1.as_slice(), nums2.as_slice());
        if n & 1 == 1 { // odd
            Self::kth(x, y, n/2 + 1) // k >= 2
        } else {
            //                    k >= 1                         k >= 2
            (Self::kth(x, y, n/2) + Self::kth(x, y, n/2 + 1))/2.0
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
        assert_eq!(Solution::find_median_sorted_arrays(vec![2], vec![]), 2.0);
    }
}
