pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .into_iter()
        .enumerate()
        .map(|(row, st)| {
            st.chars()
                .into_iter()
                .enumerate()
                .map(|(col, c)| match c {
                    '*' => '*',
                    _ => map_char(col, row, minefield),
                })
                .collect::<String>()
        })
        .collect()
}

pub fn map_char(col: usize, row: usize, board: &[&str]) -> char {
    //2d vector
    let two_dim_board: Vec<Vec<u8>> = board
        .iter()
        .map(|my_str| my_str.as_bytes().iter().map(|&c| c).collect::<Vec<u8>>())
        .collect();

    let row_num = two_dim_board.len() as u32;
    let col_num = two_dim_board[0].len() as u32;
    let directions: [(i8, i8); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    let relevant_indices: Vec<(u32, u32)> = directions
        .iter()
        .map(|val| (row as i32 + val.0 as i32, col as i32 + val.1 as i32))
        .filter(|val| val.0 >= 0 && val.1 >= 0)
        .filter(|val| (val.0 as u32) < (row_num as u32) && (val.1 as u32) < col_num as u32)
        .map(|(x, y)| (x as u32, y as u32))
        .collect();

    let count = relevant_indices
        .iter()
        .filter(|&&val| two_dim_board[val.0 as usize][val.1 as usize] as char == '*')
        .collect::<Vec<&(u32, u32)>>()
        .len();

    match count {
        1.. => std::char::from_digit(count as u32, 10).unwrap(),
        _ => ' ',
    }
}
