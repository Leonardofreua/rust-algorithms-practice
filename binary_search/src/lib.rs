pub mod binary_search;

#[cfg(test)]
mod tests_binary_search {
    use super::binary_search::binary_search;

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

#[cfg(test)]
mod tests_binary_search_recursive {
    use super::binary_search::binary_search_rec;

    const LEFT: usize = 0;

    #[test]
    fn fail_empty_list() {
        let list_of_items = vec![];
        assert_eq!(
            binary_search_rec(&list_of_items, &1, &LEFT, &list_of_items.len()),
            None
        );
    }

    #[test]
    fn success_one_item() {
        let list_of_items = vec![30];
        assert_eq!(
            binary_search_rec(&list_of_items, &30, &LEFT, &list_of_items.len()),
            Some(0)
        );
    }

    #[test]
    fn success_search_strings() {
        let say_hello_list = vec!["hi", "olá", "salut"];
        let right = say_hello_list.len();
        assert_eq!(
            binary_search_rec(&say_hello_list, &"hi", &LEFT, &right),
            Some(0)
        );
        assert_eq!(
            binary_search_rec(&say_hello_list, &"salut", &LEFT, &right),
            Some(2)
        );
    }

    #[test]
    fn fail_search_strings() {
        let say_hello_list = vec!["hi", "olá", "salut"];
        for target in &["adiós", "你好"] {
            assert_eq!(
                binary_search_rec(&say_hello_list, target, &LEFT, &say_hello_list.len()),
                None
            );
        }
    }

    #[test]
    fn success_search_integers() {
        let integers = vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90];
        for (index, element) in integers.iter().enumerate() {
            assert_eq!(
                binary_search_rec(&integers, element, &LEFT, &integers.len()),
                Some(index)
            )
        }
    }

    #[test]
    fn fail_search_integers() {
        let integers = vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90];
        for target in &[100, 444, 336] {
            assert_eq!(
                binary_search_rec(&integers, target, &LEFT, &integers.len()),
                None
            );
        }
    }

    #[test]
    fn fail_search_unsorted_strings_list() {
        let unsorted_strings = vec!["salut", "olá", "hi"];
        for target in &["hi", "salut"] {
            assert_eq!(
                binary_search_rec(&unsorted_strings, target, &LEFT, &unsorted_strings.len()),
                None
            );
        }
    }

    #[test]
    fn fail_search_unsorted_integers_list() {
        let unsorted_integers = vec![90, 80, 70, 60, 50, 40, 30, 20, 10, 0];
        for target in &[0, 80, 90] {
            assert_eq!(
                binary_search_rec(&unsorted_integers, target, &LEFT, &unsorted_integers.len()),
                None
            );
        }
    }

    #[test]
    fn success_search_string_in_middle_of_unsorted_list() {
        let unsorted_strings = vec!["salut", "olá", "hi"];
        assert_eq!(
            binary_search_rec(&unsorted_strings, &"olá", &LEFT, &unsorted_strings.len()),
            Some(1)
        );
    }

    #[test]
    fn success_search_integer_in_middle_of_unsorted_list() {
        let unsorted_integers = vec![90, 80, 70];
        assert_eq!(
            binary_search_rec(&unsorted_integers, &80, &LEFT, &unsorted_integers.len()),
            Some(1)
        );
    }
}
