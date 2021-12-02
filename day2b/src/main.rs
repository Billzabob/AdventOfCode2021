fn main() {
    let (a, b, _c) = include_str!("../input.txt")
        .lines()
        .map(|a| a.split_once(" ").unwrap())
        .map(|(direction, amount)| (direction, amount.parse::<usize>().unwrap()))
        .fold(
            (0, 0, 0),
            |(depth, forward, aim), (direction, amount)| {
                match direction {
                    "up" => (depth, forward, aim - amount),
                    "down" => (depth, forward, aim + amount),
                    _forward => (depth + (aim * amount), forward + amount, aim),
                }
            },
        );

    println!("{}", a * b)
}
