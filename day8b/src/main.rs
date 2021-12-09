use std::convert::TryInto;

fn main() {
    let init: usize = include_str!("../input.txt")
        .lines()
        .map(|n| {
            let (first, second) = n.split_once("|").unwrap();
            let first = first
                .trim()
                .split(" ")
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let second = second
                .trim()
                .split(" ")
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            find_result(first, second)
        })
        .sum();

    println!("{:?}", init);
}

fn find_result(first: [&str; 10], second: [&str; 4]) -> usize {
    let one = first.iter().find(|w| w.len() == 2).unwrap();
    let four = first.iter().find(|w| w.len() == 4).unwrap();

    second
        .iter()
        .map(|w| match w.len() {
            2 => 1,
            4 => 4,
            3 => 7,
            7 => 8,
            len => match (
                len,
                w.as_bytes()
                    .iter()
                    .filter(|c| !one.as_bytes().contains(c))
                    .count(),
                w.as_bytes()
                    .iter()
                    .filter(|c| !four.as_bytes().contains(c))
                    .count(),
            ) {
                (5, 4, 3) => 2,
                (5, 3, 2) => 3,
                (5, 4, 2) => 5,
                (6, 5, 3) => 6,
                (6, 4, 3) => 0,
                (6, 4, 2) => 9,
                _ => unreachable!(),
            },
        })
        .fold(0, |acc, n| acc * 10 + n)
}
