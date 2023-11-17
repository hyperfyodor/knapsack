use std::{
    fmt::Debug,
    collections::HashSet,
    ops::{Add, Div, Mul, Sub},
};
pub trait KnapsackItem: Clone {
    type Profit: Add<Output = Self::Profit> + PartialOrd + Clone + Debug;
    type Weight: PartialOrd + Sub<Output = Self::Weight> + Clone + Debug;
    type Bound: Add<Output = Self::Bound>
        + Mul<Output = Self::Bound>
        + Div<Output = Self::Bound>
        + PartialOrd
        + Clone
        + Debug;

    fn profit(&self) -> Self::Profit;
    fn weight(&self) -> Self::Weight;
    fn zero_bound() -> Self::Bound;
    fn zero_profit() -> Self::Profit;
    fn profit_into(profit: &Self::Profit) -> Self::Bound;
    fn weight_into(weight: &Self::Weight) -> Self::Bound;
}

#[derive(Clone, Debug)]
pub struct KnapsackNode<T: KnapsackItem> {
    branch: usize,
    bound: <T as KnapsackItem>::Bound,
    pub profit: <T as KnapsackItem>::Profit,
    pub capacity: <T as KnapsackItem>::Weight,
    pub packed: HashSet<usize>,
    pub skipped: HashSet<usize>,
}

fn zero_node<T: KnapsackItem>(
    sorted_items: &[(usize, T)],
    max_weight: <T as KnapsackItem>::Weight,
) -> KnapsackNode<T> {
    KnapsackNode {
        branch: 0,
        packed: HashSet::new(),
        skipped: HashSet::new(),
        profit: <T as KnapsackItem>::zero_profit(),
        capacity: max_weight.clone(),
        bound: bound(sorted_items, max_weight),
    }
}

fn bound<T: KnapsackItem>(
    sorted_items: &[(usize, T)],
    max_weight: <T as KnapsackItem>::Weight,
) -> <T as KnapsackItem>::Bound {
    let mut capacity = max_weight;
    let mut bound: <T as KnapsackItem>::Bound = <T as KnapsackItem>::zero_bound();

    for (index, item) in sorted_items {
        if item.weight() <= capacity {
            capacity = capacity - item.weight();
            bound = bound + <T as KnapsackItem>::profit_into(&item.profit());
        } else {
            bound = bound
                + <T as KnapsackItem>::weight_into(&capacity)
                    * <T as KnapsackItem>::profit_into(&item.profit())
                    / <T as KnapsackItem>::weight_into(&item.weight());
        }
    }

    bound
}

fn pack_item<T: KnapsackItem>(
    node: &KnapsackNode<T>,
    sorted_items: &[(usize, T)],
) -> Option<KnapsackNode<T>> {
    sorted_items[node.branch..]
        .first()
        .filter(|(_index, item)| item.weight() <= node.capacity)
        .map(|(index, item_to_pack)| {
            let mut next_packed = node.packed.clone();
            next_packed.insert(*index);

            let next_profit = node.profit.clone() + item_to_pack.profit();
            let next_capacity = node.capacity.clone() - item_to_pack.weight();
            let next_branch = node.branch + 1;
            let next_bound = bound(&sorted_items[next_branch..], next_capacity.clone())
                + <T as KnapsackItem>::profit_into(&next_profit);

            KnapsackNode::<T> {
                packed: next_packed,
                skipped: node.skipped.clone(),
                profit: next_profit,
                branch: next_branch,
                capacity: next_capacity,
                bound: next_bound,
            }
        })
}

fn skip_item<T: KnapsackItem>(
    node: &KnapsackNode<T>,
    sorted_items: &[(usize, T)],
) -> Option<KnapsackNode<T>> {
    sorted_items[node.branch..]
        .first()
        .map(|(index, _item_to_skip)| {
            let mut next_skipped = node.skipped.clone();
            next_skipped.insert(*index);
            let next_branch = node.branch + 1;

            KnapsackNode::<T> {
                packed: node.packed.clone(),
                skipped: next_skipped,
                profit: node.profit.clone(),
                branch: next_branch,
                capacity: node.capacity.clone(),
                bound: <T as KnapsackItem>::profit_into(&node.profit)
                    + bound(&sorted_items[next_branch..], node.capacity.clone()),
            }
        })
}

fn solve_node<T: KnapsackItem>(
    node: &KnapsackNode<T>,
    sorted_items: &[(usize, T)],
) -> KnapsackNode<T> {
    let mut stack: Vec<KnapsackNode<T>> = Vec::with_capacity(sorted_items.len() - node.branch - 1);

    let mut best_node = node.clone();

    stack.push(node.clone());

    while let Some(node) = stack.pop() {
        let l_node = pack_item(&node, &sorted_items)
            .filter(|node| node.bound >= <T as KnapsackItem>::profit_into(&best_node.profit));
        let r_node = skip_item(&node, &sorted_items)
            .filter(|node| node.bound >= <T as KnapsackItem>::profit_into(&best_node.profit));

        match (l_node, r_node) {
            (Some(l), Some(r)) => {
                if l.bound >= r.bound {
                    stack.push(r);
                    stack.push(l);
                } else {
                    stack.push(l);
                    stack.push(r);
                }
            }
            (Some(l), None) => {
                stack.push(l);
            }
            (None, Some(r)) => {
                stack.push(r);
            }
            (None, None) => {
                if node.profit > best_node.profit {
                    best_node = node;
                }
            }
        }
    }

    best_node
}

pub fn solve<T: KnapsackItem>(
    items: &[T],
    max_weight: <T as KnapsackItem>::Weight,
) -> KnapsackNode<T> {
    let mut sorted_items: Vec<(usize, T)> = items
        .iter()
        .enumerate()
        .map(|(i, t)| (i, t.clone()))
        .collect();

    sorted_items.sort_by(|a, b| {
        let a_ratio = <T as KnapsackItem>::profit_into(&a.1.profit())
            / <T as KnapsackItem>::weight_into(&a.1.weight());



        let b_ratio = <T as KnapsackItem>::profit_into(&b.1.profit())
        / <T as KnapsackItem>::weight_into(&b.1.weight());

        b_ratio.partial_cmp(&a_ratio).unwrap()
    });

    solve_node(&zero_node(&sorted_items, max_weight), &sorted_items)
}
