pub fn star1() {
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
            let (mut dx, mut dy) = new_cord(x, y, h, w, dir);
            while grid[dy][dx] == 0 {
                (dx, dy) = new_cord(dx, dy, h, w, dir);
            }
            if grid[dy][dx] == 2 {
                break 'step;
            } else {
                x = dx;
                y = dy;
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

fn new_cord(x: usize, y: usize, h: usize, w: usize, dir: i32) -> (usize, usize) {
    match dir {
        0 => ((x + 1) % w, y),
        1 => (x, (y + 1) % h),
        2 => {
            if x == 0 {
                (w - 1, y)
            } else {
                (x - 1, y)
            }
        }
        _ => {
            if y == 0 {
                (x, h - 1)
            } else {
                (x, y - 1)
            }
        }
    }
}
