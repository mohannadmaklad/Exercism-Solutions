use itertools::Itertools;

pub fn count(lines: &[&str]) -> u32 {
    let ls: Vec<(u32, String)> = lines
        .iter()
        .enumerate()
        .map(|(row, &l)| (row as u32, l.to_string()))
        .collect();

    let indices: Vec<(u32, u32, char)> = ls
        .iter()
        .enumerate()
        .map(|(row, (_, s))| {
            s.chars()
                .enumerate()
                .map(move |(col, c)| (row as u32, col as u32, c as char))
                .collect::<Vec<(u32, u32, char)>>()
        })
        .flatten()
        .collect::<Vec<(u32, u32, char)>>();

    let angles: Vec<(u32, u32)> = indices
        .iter()
        .filter(|(_, _, c)| *c == '+')
        .map(|(row, col, _)| (*row, *col))
        .collect();

    let h_lines: Vec<((u32, u32), (u32, u32))> = angles
        .iter()
        .cloned()
        .combinations(2)
        .filter(|v| v[0].0 == v[1].0 && is_hline(&indices, &(v[0].0, v[0].1), &(v[1].0, v[1].1)))
        .map(|v| (v[0], v[1]))
        .collect();

    let possible_squares: Vec<(((u32, u32), (u32, u32)), ((u32, u32), (u32, u32)))> = h_lines
        .iter()
        .combinations(2)
        .filter(|v| v[0].0 .1 == v[1].0 .1 && v[0].1 .1 == v[1].1 .1)
        .filter(|v| is_vline(&indices, &v[0].0, &v[1].0))
        .filter(|v| is_vline(&indices, &v[0].1, &v[1].1))
        .map(|v| ((v[0].0, v[0].1), (v[1].0, v[1].1)))
        .collect();

    possible_squares.len() as u32
}

fn is_hline(indices: &Vec<(u32, u32, char)>, start: &(u32, u32), end: &(u32, u32)) -> bool {
    indices
        .iter()
        .filter(|(row, col, ch)| {
            row == &start.0 && col > &start.1 && col < &end.1 && (*ch == '-' || *ch == '+')
        })
        .count() as u32
        == end.1 - start.1 - 1
}

fn is_vline(indices: &Vec<(u32, u32, char)>, start: &(u32, u32), end: &(u32, u32)) -> bool {
    indices
        .iter()
        .filter(|(row, col, ch)| {
            col == &start.1 && row > &start.0 && row < &end.0 && (*ch == '|' || *ch == '+')
        })
        .count() as u32
        == end.0 - start.0 - 1
}
