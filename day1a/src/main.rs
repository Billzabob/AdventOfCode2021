fn main() {
    let result = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<usize>>()
        .windows(2)
        .map(|n| n[0] < n[1])
        .filter(|n| *n)
        .count();

    println!("{}", result)
}
