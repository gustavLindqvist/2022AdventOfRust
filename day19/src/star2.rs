#[derive(Clone, Copy)]
struct Blueprint {
    ore: i32,
    clay: i32,
    obsi: (i32, i32),
    geo: (i32, i32),
}

#[derive(Clone)]
struct Res {
    ore: i32,
    clay: i32,
    obsi: i32,
    geo: i32,
}
#[derive(Clone)]
struct Bots {
    ore: i32,
    clay: i32,
    obsi: i32,
    geo: i32,
}

pub fn star2() {
    let mut prints = vec![];
    include_str!("test.in")
        .lines()
        .take(3)
        .map(|l| {
            l.split(" ")
                .map(|i| i.parse::<i32>().unwrap_or(-1))
                .filter(|&i| i != -1)
                .collect::<Vec<i32>>()
        })
        .for_each(|v| {
            prints.push(Blueprint {
                ore: v[0],
                clay: v[1],
                obsi: (v[2], v[3]),
                geo: (v[4], v[5]),
            });
        });
    let sum = prints
        .iter()
        .map(|p| {
            value(
                32,
                p.clone(),
                Res {
                    ore: 0,
                    clay: 0,
                    obsi: 0,
                    geo: 0,
                },
                Bots {
                    ore: 1,
                    clay: 0,
                    obsi: 0,
                    geo: 0,
                },
                &vec![false, false, false, false],
            )
        })
        .collect::<Vec<_>>();
    dbg!(sum.clone());
    dbg!(sum.iter().fold(1, |x, y| x * y));
}

fn value(time: i32, print: Blueprint, mut res: Res, bots: Bots, d: &Vec<bool>) -> i32 {
    let mut denied = d.clone();
    if time <= 0 {
        return res.geo;
    }
    if time == 30 {
        println!("work work");
    }
    let mut g = 0;
    let mut build = vec![
        !denied[0] & (res.ore >= print.ore),
        !denied[1] & (res.ore >= print.clay),
        !denied[2] & (res.ore >= print.obsi.0) & (res.clay >= print.obsi.1),
        !denied[3] & (res.ore >= print.geo.0) & (res.obsi >= print.geo.1),
    ];

    if build[3] | (bots.ore >= print.clay) | (time * print.clay <= bots.ore * time + res.ore) {
        build[0] = false;
        denied[0] = true;
    }
    if build[3] | (bots.clay >= print.obsi.1) | (time * print.obsi.1 <= bots.clay * time + res.ore)
    {
        build[1] = false;
        denied[1] = true;
    }
    if denied[2]
        | build[3]
        | (bots.obsi >= print.geo.1)
        | (time * print.clay <= bots.ore * time + res.ore)
    {
        build[2] = false;
        denied[2] = true;
    }
    if denied[3] | (time < 2) {
        build[3] = false;
        denied[3] = true;
    }

    res.ore += bots.ore;
    res.clay += bots.clay;
    res.obsi += bots.obsi;
    res.geo += bots.geo;

    if build[0] {
        let mut b = bots.clone();
        let mut r = res.clone();
        b.ore += 1;
        r.ore -= print.ore;
        g = g.max(value(time - 1, print, r, b, &denied));
    }
    if build[1] {
        let mut b = bots.clone();
        let mut r = res.clone();
        b.clay += 1;
        r.ore -= print.clay;
        g = g.max(value(time - 1, print, r, b, &denied));
    }
    if build[2] {
        let mut b = bots.clone();
        let mut r = res.clone();
        b.obsi += 1;
        r.ore -= print.obsi.0;
        r.clay -= print.obsi.1;
        g = g.max(value(time - 1, print, r, b, &denied));
    }
    if build[3] {
        let mut b = bots.clone();
        let mut r = res.clone();
        b.geo += 1;
        r.ore -= print.geo.0;
        r.obsi -= print.geo.1;
        g = g.max(value(time - 1, print, r, b, &denied));
    }

    g = g.max(value(time - 1, print, res, bots, &denied));

    g
}
