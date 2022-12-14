pub fn star2() {
    let size = 1000; //gissning
    let mut max_y = 0;
    let mut grid = vec![vec![false; size]; size];
    for l in include_str!("data.in").lines().map(|l| {
        l.split(" -> ")
            .map(|s| s.split_once(',').unwrap())
            .map(|(s1, s2)| (s1.parse::<usize>().unwrap(), s2.parse::<usize>().unwrap()))
    }) {
        let (mut last_x, mut last_y) = l.clone().next().unwrap();
        for (i_x, i_y) in l {
            max_y = max_y.max(i_y);

            let (d_x, d_y) = (
                i_x.max(last_x) - i_x.min(last_x),
                i_y.max(last_y) - i_y.min(last_y),
            );
            let (f_x, f_y) = (last_x.min(i_x), last_y.min(i_y));
            for x in f_x..=(f_x + d_x) {
                for y in f_y..=(f_y + d_y) {
                    grid[x][y] = true;
                }
            }
            (last_x, last_y) = (i_x, i_y);
        }
    }

    let mut count = 0;
    for x in 0..size {
        grid[x][max_y + 2] = true;
    }

    while !grid[500][0] {
        let (mut x, mut y) = (500, 0);
        loop {
            if !grid[x][y + 1] {
                (x, y) = (x, y + 1)
            } else if !grid[x - 1][y + 1] {
                (x, y) = (x - 1, y + 1)
            } else if !grid[x + 1][y + 1] {
                (x, y) = (x + 1, y + 1)
            } else {
                grid[x][y] = true;
                break;
            }
        }
        count += 1;
    }

    println!("{}", count);
}
