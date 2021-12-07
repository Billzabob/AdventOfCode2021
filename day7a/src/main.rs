fn main() {
    let mut init: Vec<_> = include_str!("../input.txt")
        .split(",")
        .map(|n| n.parse::<isize>().unwrap())
        .collect();

    init.sort_unstable();
    let fuel = fuel_cost(&init, init[init.len() / 2]);
    println!("{}", fuel);
}

fn fuel_cost(input: &Vec<isize>, index: isize) -> isize {
    input.iter().map(|n| (n - index).abs()).sum()
}
