pub fn star1() {
    let str = include_str!("source.txt").lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
    let w = 99;
    let h = 99;

    let mut sum = 0;
    
    for x in 0..w{
        for y in 0..h{
            if
                ((x + 1)..w).all(|i| str[y][x] > str[y][i])|
                (0..x).all(|i| str[y][x] > str[y][i]) |
                ((y + 1)..h).all(|i| str[y][x] > str[i][x]) |
                (0..y).all(|i| str[y][x] > str[i][x]) 
            {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}