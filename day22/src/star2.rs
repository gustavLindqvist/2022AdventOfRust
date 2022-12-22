pub fn star2() {
    let (map, inst) = include_str!("data.in").split_once("\n\n").unwrap();
    let chars = map
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    let w = chars.iter().map(|l| l.len()).max().unwrap();
    let h = chars.len();

    let mut grid: Vec<Vec<u8>> = vec![vec![0; w]; h];

    for y in 0..h {
        for x in 0..w {
            if x >= chars[y].len() {
                grid[y][x] = 0;
            } else {
                grid[y][x] = match chars[y][x] {
                    '.' => 1,
                    '#' => 2,
                    _ => 0,
                }
            }
        }
    }

    let steps = inst
        .split(|c| (c == 'L') | (c == 'R'))
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let dirs = inst
        .chars()
        .filter(|c| (c == &'L') | (c == &'R'))
        .collect::<Vec<_>>();
    let mut y: usize = 0;
    let mut x: usize = grid[0].iter().position(|i| i == &1).unwrap() as usize;
    let mut dir = 0;

    for i in 0..steps.len() {
        'step: for _ in 0..steps[i] {
            let (dx, dy, ddir) = new_cord(x as isize, y as isize, dir);
            if grid[dy as usize][dx as usize] == 0 {
                println!("NOOOO!");
            }
            if grid[dy as usize][dx as usize] == 2 {
                break 'step;
            } else {
                x = dx as usize;
                y = dy as usize;
                dir = ddir;
            }
        }
        if i < dirs.len() {
            if dirs[i] == 'R' {
                dir = (dir + 1) % 4;
            } else {
                dir = (dir + 3) % 4;
            }
        }
    }
    //dbg!(x, y, dir);
    println!("{}", (y + 1) * 1000 + (x + 1) * 4 + dir as usize);
}

fn new_cord(x: isize, y: isize, dir: i32) -> (isize, isize, i32) {
    //index with y/50, x/50
    let hardcode: Vec<Vec<Vec<(isize, isize, i32)>>> = vec![
        vec![
            vec![],
            //stämmer
            vec![
                (100, y, 0),
                (x, 50, 1),
                (0, (49 - y) + 100, 0),
                (0, x + 100, 0),
            ],
            //stämmer
            vec![
                (99, 149 - y, 2),
                (99, x - 50, 2),
                (99, y, 2),
                (x - 100, 199, 3),
            ],
        ],
        vec![
            vec![],
            vec![(y + 50, 49, 3), (x, 100, 1), (y - 50, 100, 1), (x, 49, 3)],
            vec![],
        ],
        vec![
            vec![(50, y, 0), (x, 150, 1), (50, (149 - y), 0), (50, x + 50, 0)],
            vec![
                (149, (149 - y), 2),
                (49, x + 100, 2),
                (49, y, 2),
                (x, 99, 3),
            ],
            vec![],
        ],
        vec![
            vec![
                (y - 100, 149, 3),
                (x + 100, 0, 1),
                (y - 100, 0, 1),
                (x, 149, 3),
            ],
            vec![],
            vec![],
        ],
    ];

    //dbg!(x, y, dir);
    if (x % 50 == 49) & (dir == 0) {
        hardcode[y as usize / 50][x as usize / 50][0]
    } else if (x % 50 == 0) & (dir == 2) {
        hardcode[y as usize / 50][x as usize / 50][2]
    } else if (y % 50 == 49) & (dir == 1) {
        hardcode[y as usize / 50][x as usize / 50][1]
    } else if (y % 50 == 0) & (dir == 3) {
        hardcode[y as usize / 50][x as usize / 50][3]
    } else {
        match dir {
            0 => (x + 1, y, dir),
            1 => (x, y + 1, dir),
            2 => (x - 1, y, dir),
            _ => (x, y - 1, dir),
        }
    }
}
