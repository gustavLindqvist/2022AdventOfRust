pub fn star1() {
    let size = 1000; //gissning
    let mut grid = vec![vec![false; size]; size];
    for l in include_str!("data.in").lines().map(|l| {
        l.split(" -> ")
            .map(|s| s.split_once(',').unwrap())
            .map(|(s1, s2)| (s1.parse::<usize>().unwrap(), s2.parse::<usize>().unwrap()))
    }) {
        let (mut l_x, mut l_y) = l.clone().next().unwrap();
        for (i_x, i_y) in l {
            for x in l_x.min(i_x)..=l_x.max(i_x) {
                for y in l_y.min(i_y)..=l_y.max(i_y) {
                    grid[x][y] = true;
                }
            }
            (l_x, l_y) = (i_x, i_y);
        }
    }

    let mut count = 0;
    loop {
        let (mut x, mut y) = (500, 0);
        while y < size - 1 {
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
        if y >= size - 1 {
            break;
        }
        count += 1;
    }
    println!("{}", count);
}
