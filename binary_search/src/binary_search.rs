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


pub fn binary_search(list_of_items: &[i32], target: &i32) -> Option<usize> {
  let mut left: i32 = 0;
  let mut right: i32 = (list_of_items.len() - 1) as i32;

  while left <= right {
      let middle: usize = ((left + right) / 2) as usize;

      let guess: &i32 = &list_of_items[middle];
      if guess == target {
          return Some(middle);
      } else if guess > target {
          right = middle as i32 - 1;
      } else {
          left = middle as i32 + 1
      }
  }
  None
}

pub fn binary_search_rec(list_of_items: &[i32], target: &i32, left: &i32, right: &i32) -> Option<usize> {
  if left > right {
      return None;
  }

  let middle: usize = ((left + right) / 2) as usize;
  let guess: &i32 = &list_of_items[middle];
  if guess < target {
      let middle = (middle as i32) + 1;
      return binary_search_rec(&list_of_items, &target, &middle, &right);
  } else if guess > target {
      let middle = (middle as i32) - 1;
      return binary_search_rec(&list_of_items, &target, &left, &middle);
  } else {
      return Some(middle);
  }
}