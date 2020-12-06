use std::io::{self, BufRead};
use std::ops::Range;

fn get_id(location: &str) -> u32 {
    if location.len() < 3 {
        return 0;
    }
    let (row, column) = location.split_at(location.len() - 3);
    get_row(row) * 8 + get_column(column)
}

fn find_missing_seat() -> u64 {
    let raw = std::fs::read_to_string("./src/input.txt").unwrap();
    let total: u64 = raw
        .strip_suffix('\n')
        .unwrap()
        .split('\n')
        .map(|r| get_id(r) as u64)
        .sum();
    let initial = (0..32).fold(0, |a, b| a + b);
    let total = total + initial;

    let expected = (0..=848).fold(0, |a, b| a + b);
    expected - total
}

fn get_row(location: &str) -> u32 {
    get(location, 'F', 'B', 127)
}

fn get_column(location: &str) -> u32 {
    get(location, 'L', 'R', 7)
}

fn half_round_up(i: u32) -> u32 {
    (i + 2 - 1) / 2
}

fn get(location: &str, lower: char, upper: char, highest: u32) -> u32 {
    let mut range = Range {
        start: 0,
        end: highest,
    };
    for half in location.chars() {
        let delta = half_round_up(range.end - range.start);
        match half {
            u if u == upper => range.start += delta,
            l if l == lower => range.end -= delta,
            character => panic!(format!(
                "unexpected char in get_column, {:?}\nExpected either {}, or {}",
                character, upper, lower
            )),
        };
    }
    match location.chars().last().unwrap() {
        u if u == upper => range.end,
        l if l == lower => range.start,
        _ => panic!("ruro raggy, rast rharacter rasn't rhat re rexpected"),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_id_test() {
        assert_eq!(get_id("BFFFBBFRRR"), 567);
        assert_eq!(get_id("FFFBBBFRRR"), 119);
        assert_eq!(get_id("BBFFBBFRLL"), 820);
    }

    #[test]
    fn get_column_test() {
        assert_eq!(get_column("RLR"), 5);
        assert_eq!(get_column("RRR"), 7);
        assert_eq!(get_column("RLL"), 4);
    }

    #[test]
    fn get_row_test() {
        assert_eq!(get_row("FBFBBFF"), 44);
        assert_eq!(get_row("BFFFBBF"), 70);
        assert_eq!(get_row("FFFBBBF"), 14);
        assert_eq!(get_row("BBFFBBF"), 102);
    }

    #[test]
    fn get_highest_id_from_input() {
        let max = std::fs::read_to_string("./src/input.txt")
            .unwrap()
            .strip_suffix('\n')
            .unwrap()
            .split('\n')
            .map(|location| get_id(location))
            .max()
            .unwrap();

        assert_eq!(max, 848);
    }

    #[test]
    fn find_missing() {
        assert_eq!(find_missing_seat(), 682);
    }
}
