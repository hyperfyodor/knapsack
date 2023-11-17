mod solver;

use crate::solver::{KnapsackItem, KnapsackNode};

#[derive(Clone, Debug)]
struct UserItem {
    profit: u32,
    weight: u32,
}

impl UserItem {
    fn items(items: &[(u32, u32)]) -> Vec<UserItem> {
        items
            .iter()
            .map(|(profit, weight)| UserItem {
                profit: *profit,
                weight: *weight,
            })
            .collect()
    }
}

impl KnapsackItem for UserItem {
    type Profit = u32;
    type Weight = u32;
    type Bound = f32;

    fn profit(&self) -> Self::Profit {
        self.profit
    }

    fn weight(&self) -> Self::Weight {
        self.weight
    }

    fn zero_bound() -> Self::Bound {
        0.0
    }

    fn zero_profit() -> Self::Profit {
        0
    }

    fn profit_into(profit: &Self::Profit) -> Self::Bound {
        profit.clone() as f32
    }

    fn weight_into(weight: &Self::Weight) -> Self::Bound {
        weight.clone() as f32
    }
}

#[cfg(test)]
mod tests {
    use crate::{solver::solve, UserItem};

    #[test]
    fn test_simple_task() {
        let items = UserItem::items(&[(10, 2), (10, 4), (12, 6), (18, 9)]);
        let max_weight = 15;

        let solution = solve(&items, max_weight);

        assert_eq!(solution.profit, 38);
    }

    #[test]
    fn test_simple_task_1() {
        let items = UserItem::items(&[(20, 13), (22, 8), (4, 4), (25, 16), (23, 15)]);
        let max_weight = 20;

        let solution = solve(&items, max_weight);

        assert_eq!(solution.profit, 29);
    }
}
