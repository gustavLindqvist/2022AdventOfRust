use std::collections::{HashMap, HashSet};

pub fn star1() {
    let mut elves = HashSet::new();

    let l: Vec<Vec<char>> = include_str!("data.in")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    for y in 0..l.len() {
        for x in 0..l[0].len() {
            if l[y][x] == '#' {
                elves.insert((x as i32, y as i32));
            }
        }
    }

    let deltas = vec![
        vec![(0, -1), (-1, -1), (1, -1)],
        vec![(0, 1), (-1, 1), (1, 1)],
        vec![(-1, 0), (-1, -1), (-1, 1)],
        vec![(1, 0), (1, -1), (1, 1)],
    ];
    for r in 0..10 {
        let mut next_state: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
        for (ex, ey) in &elves {
            let mut go = false;
            for x in -1..=1 {
                for y in -1..=1 {
                    if !((x == 0) & (y == 0)) {
                        go |= elves.contains(&(ex + x, ey + y));
                    }
                }
            }
            if go {
                let mut moved = false;
                'dir: for d in 0..4 {
                    for (dx, dy) in &deltas[(r + d) % 4] {
                        if elves.contains(&(ex + dx, ey + dy)) {
                            continue 'dir;
                        }
                    }
                    let (nx, ny) = (ex + deltas[(r + d) % 4][0].0, ey + deltas[(r + d) % 4][0].1);
                    if next_state.contains_key(&(nx, ny)) {
                        let e = next_state.remove(&(nx, ny)).unwrap();
                        next_state.insert(e, e);
                        next_state.insert((*ex, *ey), (*ex, *ey));
                    } else {
                        next_state.insert((nx, ny), (*ex, *ey));
                    }
                    moved = true;
                    break 'dir;
                }
                if !moved {
                    next_state.insert((*ex, *ey), (*ex, *ey));
                }
            } else {
                next_state.insert((*ex, *ey), (*ex, *ey));
            }
        }
        elves = next_state.keys().map(|t| *t).collect::<HashSet<_>>();
    }
    let (mut maxx, mut maxy, mut minx, mut miny) = (i32::MIN, i32::MIN, i32::MAX, i32::MAX);
    for (ex, ey) in &elves {
        maxx = maxx.max(*ex);
        maxy = maxy.max(*ey);
        minx = minx.min(*ex);
        miny = miny.min(*ey);
    }
    println!(
        "{}",
        (maxx - minx + 1) * (maxy - miny + 1) - elves.len() as i32
    );
}
