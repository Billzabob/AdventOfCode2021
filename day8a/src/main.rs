fn main() {
    let init = include_str!("../input.txt")
        .lines()
        .flat_map(|n| {
            let (_first, second) = n.split_once("|").unwrap();
            second.trim().split(" ")
        })
        .filter(|a| [2, 3, 4, 7].contains(&a.len()) )
        .count();

    println!("{:?}", init);
}