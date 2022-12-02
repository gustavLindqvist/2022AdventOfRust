fn main() {
    //part 1
    let contents = include_str!("source.txt").split("\n");
    let mut sum = 0;
    for s in contents {
        match s {
            "A X" => sum += 4,
            "B X" => sum += 1,
            "C X" => sum += 7,
            "A Y" => sum += 8,
            "B Y" => sum += 5,
            "C Y" => sum += 2,
            "A Z" => sum += 3,
            "B Z" => sum += 9,
            "C Z" => sum += 6,
            _ => sum += 0,
        }
    }
    println!("{}", sum);

    //part 2
    let contents = include_str!("source.txt").split("\n");
    sum = 0;
    for s in contents {
        match s {
            "A X" => sum += 3,
            "B X" => sum += 1,
            "C X" => sum += 2,
            "A Y" => sum += 4,
            "B Y" => sum += 5,
            "C Y" => sum += 6,
            "A Z" => sum += 8,
            "B Z" => sum += 9,
            "C Z" => sum += 7,
            _ => sum += 0,
        }
    }
    println!("{}", sum);
}
