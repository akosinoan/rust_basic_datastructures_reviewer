// ============================================================
// LeetCode #121 — Best Time to Buy and Sell Stock
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
// Difficulty: Easy
// ============================================================
//
// Given an array `prices` where prices[i] is the price on day i,
// return the maximum profit from one buy + one sell.
// You must buy before you sell. Return 0 if no profit is possible.
//
// Example:
//   [7,1,5,3,6,4] → 5  (buy at 1, sell at 6)
//   [7,6,4,3,1]   → 0  (prices only fall)
//
// ---------------------------------------------------------------
// APPROACH: One pass — O(n) time, O(1) space
//
// Track the minimum price seen so far.
// At each step: profit = current_price - min_price_so_far
// Update max_profit if this profit is better.
// ---------------------------------------------------------------

pub fn max_profit(prices: Vec<i32>) -> i32 {
    todo!(
        "Track min_price (start at i32::MAX) and max_profit (start at 0).\
         For each price: update min_price, then update max_profit."
    )
}

// ---------------------------------------------------------------
// BONUS: LeetCode #122 — Best Time to Buy and Sell Stock II
//
// You can buy and sell multiple times (but must sell before re-buying).
// Return total maximum profit.
//
// Key insight: capture every upward move.
//   if prices[i] > prices[i-1]: profit += prices[i] - prices[i-1]
// ---------------------------------------------------------------
pub fn max_profit_ii(prices: Vec<i32>) -> i32 {
    todo!(
        "For each consecutive pair, if price went up, add the difference to profit."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn example_2() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn single_day() {
        assert_eq!(max_profit(vec![5]), 0);
    }

    #[test]
    fn all_same() {
        assert_eq!(max_profit(vec![3, 3, 3, 3]), 0);
    }

    #[test]
    fn bonus_max_profit_ii() {
        assert_eq!(max_profit_ii(vec![7, 1, 5, 3, 6, 4]), 7); // (5-1) + (6-3) = 7
        assert_eq!(max_profit_ii(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(max_profit_ii(vec![7, 6, 4, 3, 1]), 0);
    }
}
