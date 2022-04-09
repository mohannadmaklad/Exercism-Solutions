pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 || array[0] > key || array[array.len() - 1] < key {
        return None;
    }

    let mut min_index = 0;
    let mut max_index = array.len() - 1;
    let mut current_index;

    while min_index <= max_index {
        current_index = (min_index + max_index) / 2;

        match array[current_index] {
            v if v > key => max_index = current_index - 1,
            v if v < key => min_index = current_index + 1,
            _ => return Some(current_index),
        }
    }
    None
}
