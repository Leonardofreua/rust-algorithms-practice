pub mod binary_search;

pub static LIST_SIZE_1: &[i32] = &[30];
pub static LIST_SIZE_2: &[i32] = &[34, 55];
pub static LIST_SIZE_3: &[i32] = &[111, 202, 566];
pub static LIST_SIZE_10: &[i32] = &[0, 10, 20, 30, 40, 50, 60, 70, 80, 90];

#[cfg(test)]
mod tests_binary_search {
    use super::*;
    use super::binary_search::binary_search;
    use rstest::rstest;

    #[test]
    fn success_search_element_list_size_1() {
        assert_eq!(
            binary_search(LIST_SIZE_1, &30),
            Some(0)
        );
    }

    #[rstest]
    #[case(99)]
    #[case(-99)]
    fn fail_search_for_non_existent_element_list_size_1(#[case] target: i32) {
        assert_eq!(
            binary_search(LIST_SIZE_1, &target),
            None
        )
    }

    #[test]
    fn success_search_elements_list_size_2() {
        for (index, element) in LIST_SIZE_2.iter().enumerate() {
            assert_eq!(
                binary_search(LIST_SIZE_2, element),
                Some(index)
            )
        }
    }

    #[rstest]
    #[case(35)]
    #[case(-55)]
    fn fail_search_for_non_existent_elements_list_size_2(#[case] target: i32) {
        assert_eq!(
            binary_search(LIST_SIZE_2, &target),
            None
        )
    }

    #[test]
    fn fail_search_unsorted_list_size_2() {
        let mut list_size_2_clone= LIST_SIZE_2.clone().to_owned();
        list_size_2_clone.reverse();
        assert_eq!(
            binary_search(&list_size_2_clone, &34),
            None
        );
    }

    #[test]
    fn success_search_elements_list_size_3() {
        for (index, element) in LIST_SIZE_3.iter().enumerate() {
            assert_eq!(
                binary_search(LIST_SIZE_3, element),
                Some(index)
            )
        }
    }

    #[rstest]
    #[case(110)]
    #[case(-111)]
    #[case(567)]
    fn fail_search_for_non_existents_list_size_3(#[case] target: i32) {
        assert_eq!(
            binary_search(LIST_SIZE_3, &target),
            None
        );
    }

    #[test]
    fn fail_search_unsorted_list_size_3() {
        let mut list_size_3_clone= LIST_SIZE_3.clone().to_owned();
        list_size_3_clone.reverse();
        assert_eq!(
            binary_search(&list_size_3_clone, &111),
            None
        );
    }

    #[test]
    fn success_search_elements_list_size_10() {
        for (index, element) in LIST_SIZE_10.iter().enumerate() {
            assert_eq!(
                binary_search(LIST_SIZE_10, element),
                Some(index)
            )
        }
    }

    #[rstest]
    #[case(-10)]
    #[case(11)]
    #[case(100)]
    fn fail_search_for_non_existents_list_size_10(#[case] target: i32) {
        assert_eq!(
            binary_search(LIST_SIZE_10, &target),
            None
        );
    }

    #[test]
    fn fail_search_unsorted_list_size_10() {
        let mut list_size_10_clone= LIST_SIZE_10.clone().to_owned();
        list_size_10_clone.reverse();
        assert_eq!(
            binary_search(&list_size_10_clone, &10),
            None
        );
    }
}


#[cfg(test)]
mod tests_binary_search_recursive {
    use super::*;
    use super::binary_search::binary_search_rec;
    use rstest::rstest;

    const LEFT: i32 = 0;

    fn _get_right_value_from_list(list: &[i32]) -> i32 {
        (list.len() - 1) as i32
    }

    #[test]
    fn success_search_element_list_size_1() {
        let right: i32 = _get_right_value_from_list(LIST_SIZE_1);
        let search_result = binary_search_rec(LIST_SIZE_1, &30, &LEFT, &right);
        assert_eq!(search_result, Some(0));
    }

    #[rstest]
    #[case(99)]
    #[case(-99)]
    fn fail_search_for_non_existent_element_list_size_1(#[case] target: i32) {
        let right: i32 = _get_right_value_from_list(LIST_SIZE_1);
        let search_result = binary_search_rec(LIST_SIZE_1, &target, &LEFT, &right);
        assert_eq!(search_result, None);
    }

    #[test]
    fn success_search_elements_list_size_2() {
        let right: i32 = _get_right_value_from_list(LIST_SIZE_2);
        for (index, element) in LIST_SIZE_2.iter().enumerate() {
            assert_eq!(
                binary_search_rec(LIST_SIZE_2, element, &LEFT, &right),
                Some(index)
            );
        }
    }

    #[rstest]
    #[case(35)]
    #[case(-55)]
    fn fail_search_for_non_existent_elements_list_size_2(#[case] target: i32) {
        let right: i32 = _get_right_value_from_list(LIST_SIZE_2);
        assert_eq!(
            binary_search_rec(LIST_SIZE_2, &target, &LEFT, &right),
            None
        )
    }

    #[test]
    fn fail_search_unsorted_list_size_2() {
        let mut list_size_2_clone= LIST_SIZE_2.clone().to_owned();
        list_size_2_clone.reverse();
        let right: i32 = _get_right_value_from_list(&list_size_2_clone);
        assert_eq!(
            binary_search_rec(&list_size_2_clone, &34, &LEFT, &right),
            None
        );
    }

    #[test]
    fn success_search_elements_list_size_3() {
        let right: i32 = _get_right_value_from_list(LIST_SIZE_3);
        for (index, element) in LIST_SIZE_3.iter().enumerate() {
            assert_eq!(
                binary_search_rec(LIST_SIZE_3, element, &LEFT, &right),
                Some(index)
            );
        }
    }

    #[rstest]
    #[case(110)]
    #[case(-111)]
    #[case(567)]
    fn fail_search_for_non_existent_elements_list_size_3(#[case] target: i32) {
        let right: i32 = _get_right_value_from_list(LIST_SIZE_3);
        assert_eq!(
            binary_search_rec(LIST_SIZE_3, &target, &LEFT, &right),
            None
        )
    }

    #[test]
    fn fail_search_unsorted_list_size_3() {
        let mut list_size_3_clone= LIST_SIZE_3.clone().to_owned();
        list_size_3_clone.reverse();
        let right: i32 = _get_right_value_from_list(&list_size_3_clone);
        assert_eq!(
            binary_search_rec(&list_size_3_clone, &34, &LEFT, &right),
            None
        );
    }

    #[test]
    fn success_search_elements_list_size_10() {
        let right: i32 = _get_right_value_from_list(LIST_SIZE_10);
        for (index, element) in LIST_SIZE_10.iter().enumerate() {
            assert_eq!(
                binary_search_rec(LIST_SIZE_10, element, &LEFT, &right),
                Some(index)
            );
        }
    }

    #[rstest]
    #[case(-10)]
    #[case(11)]
    #[case(100)]
    fn fail_search_for_non_existent_elements_list_size_10(#[case] target: i32) {
        let right: i32 = _get_right_value_from_list(LIST_SIZE_10);
        assert_eq!(
            binary_search_rec(LIST_SIZE_10, &target, &LEFT, &right),
            None
        )
    }

    #[test]
    fn fail_search_unsorted_list_size_10() {
        let mut list_size_10_clone= LIST_SIZE_10.clone().to_owned();
        list_size_10_clone.reverse();
        let right: i32 = _get_right_value_from_list(&list_size_10_clone);
        assert_eq!(
            binary_search_rec(&list_size_10_clone, &34, &LEFT, &right),
            None
        );
    }
}
