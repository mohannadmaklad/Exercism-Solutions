#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    let first_list_len = first_list.len();
    let second_list_len = second_list.len();

    match (first_list_len, second_list_len) {
        (0, 0) => return Comparison::Equal,
        (0, _) => return Comparison::Sublist,
        (_, 0) => return Comparison::Superlist,
        (_, _) => (),
    }

    if first_list_len < second_list_len && is_sub_list(first_list, second_list) {
        return Comparison::Sublist;
    } else if first_list_len > second_list_len && is_sub_list(second_list, first_list) {
        return Comparison::Superlist;
    } else {
        return Comparison::Unequal;
    }
}

fn is_sub_list<T: PartialEq>(small_list: &[T], big_list: &[T]) -> bool {
    let start_indices: Vec<usize> = big_list
        .iter()
        .enumerate()
        .filter(|(_, val)| small_list.iter().nth(1).unwrap() == *val)
        .map(|(i, _)| i)
        .collect();

    for val in start_indices.iter() {
        if is_equal(big_list, small_list, *val as u32) {
            return true;
        }
    }

    return false;
}

fn is_equal<T: PartialEq>(big_list: &[T], small_list: &[T], start_index: u32) -> bool {
    let small_list_len = small_list.len();
    let skipped = start_index - start_index.min(1);

    if big_list.len() < start_index as usize + small_list_len - 1 {
        return false;
    }

    //Big List Iterator
    let mut big_list_iter = big_list.iter().skip(skipped as usize);

    //Length of Matching elements
    let matching_elements_num: usize = small_list
        .iter()
        .take_while(|x| *x == big_list_iter.next().unwrap())
        .collect::<Vec<&T>>()
        .len();

    if matching_elements_num == small_list_len {
        return true;
    }

    false
}
