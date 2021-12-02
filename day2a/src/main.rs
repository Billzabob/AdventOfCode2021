fn main() {
    let (a, b) = include_str!("../input.txt")
        .lines()
        .map(|a| a.split_once(" ").unwrap())
        .map(|(direction, amount)| (direction, amount.parse::<usize>().unwrap()))
        .fold(
            (0, 0),
            |(depth, forward), (direction, amount)| match direction {
                "up" => (depth - amount, forward),
                "down" => (depth + amount, forward),
                _forward => (depth, forward + amount),
            },
        );

    println!("{}", a * b)
}
