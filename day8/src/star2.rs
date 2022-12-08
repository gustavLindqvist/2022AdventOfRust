pub fn star2() {
    println!("{}", [include_str!("source.txt").lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>()).collect::<Vec<_>>()
    ].iter().map(|str| (0..str.len()).map(|y| {
        (0..str[0].len()).map(|x| {
            (y - (0..y)
            .rev()
            .find(|&i| str[i][x] >= str[y][x])
            .unwrap_or(0)) * ((y + 1..str.len())
            .find(|&i| str[i][x] >= str[y][x])
            .unwrap_or(str.len() - 1) - y) * (x - (0..x)
            .rev()
            .find(|&i| str[y][i] >= str[y][x])
            .unwrap_or(0)) * ((x + 1..str[0].len())
            .find(|&i| str[y][i] >= str[y][x])
            .unwrap_or(str[0].len() - 1) - x)
        }
    ).max().unwrap_or(0)
}).max().unwrap_or(0)).collect::<Vec<_>>()[0]);
}
