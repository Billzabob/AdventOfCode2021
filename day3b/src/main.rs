const WIDTH: usize = 12;

fn main() {
    let lines: Vec<&str> = include_str!("../input.txt").lines().collect();

    let oxy = run(lines.clone(), |lines_left, i| {
        let zeros = lines_left
            .iter()
            .filter(|line| line.as_bytes()[i] == '0' as u8)
            .count();
        if zeros > lines_left.len() / 2 {
            '0' as u8
        } else {
            '1' as u8
        }
    });

    let co2 = run(lines, |lines_left, i| {
        let zeros = lines_left
            .iter()
            .filter(|line| line.as_bytes()[i] == '0' as u8)
            .count();
        if zeros > lines_left.len() / 2 {
            '1' as u8
        } else {
            '0' as u8
        }
    });

    println!("{}", co2 * oxy);
}

fn run<P>(lines: Vec<&str>, char_to_keep: P) -> u32
where
    P: Fn(&Vec<&str>, usize) -> u8,
{
    let g = (0..WIDTH)
        .scan(lines.clone(), |lines, i| {
            let char = char_to_keep(&lines, i);
            lines.retain(|line| line.as_bytes()[i] == char as u8);
            Some(lines.clone())
        })
        .filter(|a| a.len() > 0)
        .last()
        .unwrap();

    let c = g.first().unwrap();
    u32::from_str_radix(c, 2).unwrap()
}
