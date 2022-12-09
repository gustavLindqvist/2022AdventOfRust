use std::collections::HashSet;

pub fn star2() {
    let mut head: Vec<(i32, i32)> = vec![(0, 0)];
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
            let last = head.last().unwrap();
            head.push((last.0 + up.0, last.1 + up.1));
        }
    }
    let mut rope = vec![head];
    for i in 1..10 {
        let last = rope[i - 1].clone();
        rope.push(knot_move(last));
    }
    let pos = rope[9].iter().collect::<HashSet<_>>().len();
    println!("{}", pos);
}

fn knot_move(head: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut history: Vec<(i32, i32)> = vec![(0, 0)];
    for h in head {
        let last = history.last().unwrap();
        let diff = (h.0 - last.0, h.1 - last.1);
        if (i32::abs(diff.0) > 1) | (i32::abs(diff.1) > 1) {
            history.push((last.0 + i32::signum(diff.0), last.1 + i32::signum(diff.1)));
        } else {
            history.push(*history.last().unwrap());
        }
    }
    history
}
