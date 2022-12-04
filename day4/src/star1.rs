pub fn star1() {
    println!(
        "{}",
        include_str!("source.txt")
            .split("\n")
            .map(|l| l.split_once(',').unwrap())
            .map(|(t1, t2)| (
                t1.split_once('-')
                    .map(|p| (p.0.parse::<i32>().unwrap(), p.1.parse::<i32>().unwrap()))
                    .unwrap(),
                t2.split_once('-')
                    .map(|p| (p.0.parse::<i32>().unwrap(), p.1.parse::<i32>().unwrap()))
                    .unwrap()
            ))
            .filter(|((t11, t12), (t21, t22))| ((t11 >= t21) & (t12 <= t22))
                | ((t11 <= t21) & (t12 >= t22)))
            .collect::<Vec<_>>()
            .len()
    );
}
