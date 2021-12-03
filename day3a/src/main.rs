fn main() {
    let result = include_str!("../input.txt")
        .lines()
        .map(|a| {
            a.chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .fold(vec![0; 12], |so_far, v| {
            v.iter().zip(so_far).map(|(a, b)| a + b).collect()
        });

    let result = result
        .iter()
        .map(|count| if count > &500 { 1u8 } else { 0u8 })
        .collect::<Vec<u8>>();

    let inverse = result
        .iter()
        .map(|a| if a == &1u8 { 0u8 } else { 1u8 })
        .collect::<Vec<u8>>();

    println!("{}", to_u32(&result) * to_u32(&inverse))
}

fn to_u32(slice: &[u8]) -> u32 {
    slice.iter().fold(0, |acc, &b| acc * 2 + b as u32)
}
