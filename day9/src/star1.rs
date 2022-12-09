use std::collections::HashSet;

pub fn star1() {
    let mut history: Vec<(i32, i32)> = vec![(0, 0)];
    let mut diff = (0, 0);
    for (dir, amp) in include_str!("source.txt")
        .lines()
        .map(|l| l.split_once(" ").unwrap())
    {
        for _ in 0..(amp.parse::<i32>().unwrap()) {
            let up = match dir {
                "R" => (1, 0),
                "L" => (-1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => unreachable!(),
            };
            diff = (diff.0 + up.0, diff.1 + up.1);
            if (i32::abs(diff.0) > 1) | (i32::abs(diff.1) > 1) {
                let last = history.last().unwrap();
                history.push((last.0 + i32::signum(diff.0), last.1 + i32::signum(diff.1)));
                diff = (diff.0 - i32::signum(diff.0), diff.1 - i32::signum(diff.1));
            } else {
                history.push(*history.last().unwrap());
            }
        }
    }
    println!("{}", history.iter().collect::<HashSet<_>>().len());
}
