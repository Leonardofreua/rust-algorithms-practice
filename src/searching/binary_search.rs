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

#[cfg(test)]
mod tests_binary_search {
    use super::binary_search;

    #[test]
    fn fail_empty_list() {
        assert_eq!(binary_search(&[], &1), None);
    }

    #[test]
    fn success_one_item() {
        assert_eq!(binary_search(&[30], &30), Some(0));
    }

    #[test]
    fn success_search_strings() {
        let say_hello_list = vec!["hi", "olá", "salut"];
        assert_eq!(binary_search(&say_hello_list, &"hi"), Some(0));
        assert_eq!(binary_search(&say_hello_list, &"salut"), Some(2));
    }

    #[test]
    fn fail_search_strings() {
        let say_hello_list = vec!["hi", "olá", "salut"];
        for target in &["adiós", "你好"] {
            assert_eq!(binary_search(&say_hello_list, target), None);
        }
    }

    #[test]
    fn success_search_integers() {
        let integers = vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90];
        for (index, element) in integers.iter().enumerate() {
            assert_eq!(binary_search(&integers, element), Some(index))
        }
    }

    #[test]
    fn fail_search_integers() {
        let integers = vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90];
        for target in &[100, 444, 326] {
            assert_eq!(binary_search(&integers, target), None);
        }
    }

    #[test]
    fn fail_search_unsorted_strings_list() {
        let unsorted_strings = vec!["salut", "olá", "hi"];
        for target in &["hi", "salut"] {
            assert_eq!(binary_search(&unsorted_strings, target), None);
        }
    }

    #[test]
    fn fail_search_unsorted_integers_list() {
        let unsorted_integers = vec![90, 80, 70, 60, 50, 40, 30, 20, 10, 0];
        for target in &[0, 80, 90] {
            assert_eq!(binary_search(&unsorted_integers, target), None);
        }
    }

    #[test]
    fn success_search_string_in_middle_of_unsorted_list() {
        let unsorted_strings = vec!["salut", "olá", "hi"];
        assert_eq!(binary_search(&unsorted_strings, &"olá"), Some(1));
    }

    #[test]
    fn success_search_integer_in_middle_of_unsorted_list() {
        let unsorted_strings = vec![90, 80, 70];
        assert_eq!(binary_search(&unsorted_strings, &80), Some(1));
    }
}