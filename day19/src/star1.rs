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

pub fn star1() {
    let mut prints = vec![];
    include_str!("data.in")
        .lines()
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
    let sum: i32 = prints
        .iter()
        .enumerate()
        .map(|(i, p)| {
            (i + 1) as i32
                * value(
                    24,
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
                )
        })
        .sum();
    dbg!(sum);
}

fn value(time: i32, print: Blueprint, mut res: Res, bots: Bots) -> i32 {
    if time <= 0 {
        return res.geo;
    }
    if time == 22 {
        println!("work work");
    }
    let mut g = 0;
    let mut build = vec![
        res.ore >= print.ore,
        res.ore >= print.clay,
        (res.ore >= print.obsi.0) & (res.clay >= print.obsi.1),
        (res.ore >= print.geo.0) & (res.obsi >= print.geo.1),
    ];

    if time < 10 {
        build[0] = false;
    }
    if time < 5 {
        build[1] = false;
    }
    if time < 3 {
        build[2] = false;
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
        g = g.max(value(time - 1, print, r, b));
    }
    if build[1] {
        let mut b = bots.clone();
        let mut r = res.clone();
        b.clay += 1;
        r.ore -= print.clay;
        g = g.max(value(time - 1, print, r, b));
    }
    if build[2] {
        let mut b = bots.clone();
        let mut r = res.clone();
        b.obsi += 1;
        r.ore -= print.obsi.0;
        r.clay -= print.obsi.1;
        g = g.max(value(time - 1, print, r, b));
    }
    if build[3] {
        let mut b = bots.clone();
        let mut r = res.clone();
        b.geo += 1;
        r.ore -= print.geo.0;
        r.obsi -= print.geo.1;
        g = g.max(value(time - 1, print, r, b));
    }

    g = g.max(value(time - 1, print, res, bots));

    g
}
