///  Observations:
///     * The item list must be sorted;
///     * Returns the position of the item in the list;
///
///
///
///             LEFT               MIDDLE              RIGHT
///              \/                  \/                  \/
///  indexes:   [0]  [1]  [2]  [3]  [4]  [5]  [6]  [7]  [8]
///            |--------------------------------------------|
///  values:   | 20 | 23 | 25 | 28 | 35 | 36 | 40 | 41 | 42 |
///            |--------------------------------------------|
///
///
///  Running Time:
///     * Best-case O(1)
///     * Average O(log n)
///     * Worst-case O(log n)
use std::cmp::Ordering;

pub fn binary_search<T: Ord>(list_of_items: &[T], target: &T) -> Option<usize> {
    let mut left: usize = 0;
    let mut right: usize = list_of_items.len();

    while left < right {
        let middle: usize = left + (right - left) / 2;

        match target.cmp(&list_of_items[middle]) {
            Ordering::Less => right = middle,
            Ordering::Greater => left = middle + 1,
            Ordering::Equal => return Some(middle),
        }
    }
    None
}

pub fn binary_search_rec<T: Ord>(
    list_of_items: &[T],
    target: &T,
    left: &usize,
    right: &usize,
) -> Option<usize> {
    if left >= right {
        return None;
    }

    let middle: usize = left + (right - left) / 2;
    match target.cmp(&list_of_items[middle]) {
        Ordering::Less => binary_search_rec(list_of_items, target, left, &middle),
        Ordering::Greater => binary_search_rec(list_of_items, target, &(middle + 1), right),
        Ordering::Equal => Some(middle),
    }
}
