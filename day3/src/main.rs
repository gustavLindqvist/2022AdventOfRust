fn main() {
    let lines = include_str!("source.txt").split("\n").collect::<Vec<_>>();

    let mut p1: Vec<i32> = vec![];
    let mut p2: Vec<i32> = vec![];
    for i in 0..lines.len() {
        let (a, b) = lines[i].split_at(lines[i].len() / 2);
        for c in a.chars() {
            if b.contains(c) {
                p1.push(c as i32);
                break;
            }
        }
        if i % 3 == 0 {
            let l0 = lines[i];
            let l1 = lines[i + 1];
            let l2 = lines[i + 2];
            for c in l0.chars() {
                if l1.contains(c) && l2.contains(c) {
                    p2.push(c as i32);
                    break;
                }
            }
        }
    }

    let sum1: i32 = p1.iter().map(|c| prio(*c)).sum();
    let sum2: i32 = p2.iter().map(|c| prio(*c)).sum();
    println!("{}\n{}", sum1, sum2);
}

fn prio(c: i32) -> i32 {
    if c > 90 {
        return c % 32;
    } else {
        return (c % 32) + 26;
    }
}
