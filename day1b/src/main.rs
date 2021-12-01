fn main() {
    let result = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>()
        .windows(4)
        .map(|n| n[0] < n[3])
        .filter(|n| *n)
        .count();

    println!("{}", result)
}
