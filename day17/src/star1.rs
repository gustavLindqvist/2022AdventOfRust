pub fn star1() {
    let p1 = vec![
        vec![false, false, false, false],
        vec![false, false, false, false],
        vec![false, false, false, false],
        vec![true, true, true, true],
    ];
    let p2 = vec![
        vec![false, false, false, false],
        vec![false, true, false, false],
        vec![true, true, true, false],
        vec![false, true, false, false],
    ];
    let p3 = vec![
        vec![false, false, false, false],
        vec![false, false, true, false],
        vec![false, false, true, false],
        vec![true, true, true, false],
    ];
    let p4 = vec![
        vec![true, false, false, false],
        vec![true, false, false, false],
        vec![true, false, false, false],
        vec![true, false, false, false],
    ];
    let p5 = vec![
        vec![false, false, false, false],
        vec![false, false, false, false],
        vec![true, true, false, false],
        vec![true, true, false, false],
    ];

    let pat = vec![p1, p2, p3, p4, p5];

    let mut stack = vec![vec![true; 15]];
    for _ in 0..8088 {
        stack.push(vec![
            true, true, true, true, false, false, false, false, false, false, false, true, true,
            true, true,
        ]);
    }

    let inst = include_str!("data.in").chars().collect::<Vec<char>>();
    let mut peak = 1;
    let mut i = 0;
    for p in 0..2022 {
        // println!("rock: {}, {}", p, peak);
        // for y in (0..10).rev() {
        //     for x in 2..11 {
        //         if !stack[y][x] {
        //             print!(".");
        //         } else {
        //             print!("#");
        //         }
        //     }
        //     println!("");
        // }
        // println!("");
        let mut x = 6;
        let mut y = peak + 3;
        let mut tx = x;
        let mut ty = y;
        'fall: loop {
            if inst[i] == '<' {
                // println!("<");
                x -= 1;
            } else {
                // println!(">");
                x += 1;
            }
            i = (i + 1) % inst.len();

            let mut works = true;
            for py in 0..4 {
                for px in 0..4 {
                    if stack[y + py][x + px] & pat[p % 5][3 - py][px] {
                        works = false;
                    }
                }
            }
            if works {
                tx = x;
            } else {
                // println!("stopped");
                x = tx;
            }

            y -= 1;
            // println!("down");
            for py in 0..4 {
                for px in 0..4 {
                    if stack[y + py][x + px] & pat[p % 5][3 - py][px] {
                        break 'fall;
                    }
                }
            }
            ty = y;
        }
        for py in 0..4 {
            for px in 0..4 {
                stack[ty + py][tx + px] |= pat[p % 5][3 - py][px];
            }
        }

        for cy in (peak..=peak + 7).rev() {
            let mut t = false;
            for cx in 4..=10 {
                t = t | stack[cy][cx];
            }
            if t {
                peak = cy + 1;
                break;
            }
        }
    }

    println!("{}", peak - 1);
}
