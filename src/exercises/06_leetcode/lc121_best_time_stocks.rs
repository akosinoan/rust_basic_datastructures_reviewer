// ============================================================
// LeetCode #121 — Best Time to Buy and Sell Stock
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
// Difficulty: Easy
// ============================================================
//
// Given an array `prices` where prices[i] is the price on day i,
// return the maximum profit from one buy + one sell. You must buy
// before you sell. Return 0 if no profit is possible.
//
// Examples:
//   [7,1,5,3,6,4] → 5   (buy day 1 @ 1, sell day 4 @ 6)
//   [7,6,4,3,1]   → 0   (prices only fall — never profitable)

// --- max_profit ---
// LeetCode #121: ONE buy followed by ONE sell.
//
// Inputs:  prices — owned Vec<i32>, length ≥ 1.
// Returns: i32 — best profit achievable, or 0 if no opportunity exists.
//          Aim for O(n) time, O(1) space.
//
// Edge cases the tests check:
//   - [7,1,5,3,6,4]   → 5
//   - [7,6,4,3,1]     → 0   (monotone decreasing)
//   - [5]             → 0   (only one day, can't both buy and sell)
//   - [3,3,3,3]       → 0   (no movement)
pub fn max_profit(prices: Vec<i32>) -> i32 {
    todo!()
}

// ---------------------------------------------------------------
// BONUS: LeetCode #122 — Buy and Sell Stock II
//
// You may buy and sell as many times as you like, but you must sell
// before you re-buy. Return the total maximum profit.
// ---------------------------------------------------------------

// --- max_profit_ii ---
// LeetCode #122: unlimited transactions, no overlap.
//
// Inputs:  prices — owned Vec<i32>.
// Returns: i32 — total profit if you capture every upward move.
//
// Edge cases the tests check:
//   - [7,1,5,3,6,4]   → 7   (4 from buy@1/sell@5, 3 from buy@3/sell@6)
//   - [1,2,3,4,5]     → 4   (one steady climb)
//   - [7,6,4,3,1]     → 0   (no upward moves)
pub fn max_profit_ii(prices: Vec<i32>) -> i32 {
    todo!()
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
