fn main() {
    let init: Vec<_> = include_str!("../input.txt")
        .split(",")
        .map(|n| n.parse::<isize>().unwrap())
        .collect();

    let start = init.iter().min().unwrap();
    let end = init.iter().max().unwrap();

    let mut best = fuel_cost(&init, 0);

    for i in *start..*end {
        let a = fuel_cost(&init, i);
        if a < best {best = a}
    }

    println!("{}", best);
}

fn fuel_cost(input: &Vec<isize>, index: isize) -> isize {
    input.iter().map(|n| {
        let diff = (n - index).abs();
        (diff * diff + diff) / 2
    }).sum()
}