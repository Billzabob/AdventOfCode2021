fn main() {
    let mut init = include_str!("../input.txt")
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .fold([0; 9], |mut a, b| {
            a[b] += 1;
            a
        });

    for i in 0..80 {
        init[(i + 7) % 9] += init[i % 9]
    }

    let result: u64 = init.iter().sum();

    println!("{}", result);
}
