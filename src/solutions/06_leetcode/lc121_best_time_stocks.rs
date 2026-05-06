pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::MAX;
    let mut max_profit = 0;
    for price in prices {
        min_price = min_price.min(price);
        max_profit = max_profit.max(price - min_price);
    }
    max_profit
}

pub fn max_profit_ii(prices: Vec<i32>) -> i32 {
    prices.windows(2).map(|w| (w[1] - w[0]).max(0)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() { assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5); }

    #[test]
    fn example_2() { assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0); }

    #[test]
    fn single_day() { assert_eq!(max_profit(vec![5]), 0); }

    #[test]
    fn all_same() { assert_eq!(max_profit(vec![3, 3, 3, 3]), 0); }

    #[test]
    fn bonus_max_profit_ii() {
        assert_eq!(max_profit_ii(vec![7, 1, 5, 3, 6, 4]), 7);
        assert_eq!(max_profit_ii(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(max_profit_ii(vec![7, 6, 4, 3, 1]), 0);
    }
}
