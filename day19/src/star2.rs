#[derive(Clone, Copy)]
struct Blueprint {
    ore: u16,
    clay: u16,
    obsi: (u16, u16),
    geo: (u16, u16),
}

#[derive(Clone)]
struct Res {
    ore: u16,
    clay: u16,
    obsi: u16,
    geo: u16,
}
#[derive(Clone)]
struct Bots {
    ore: u16,
    clay: u16,
    obsi: u16,
    geo: u16,
}

pub fn star2() {
    let mut prints = vec![];
    include_str!("test.in")
        .lines()
        .take(3)
        .map(|l| {
            l.split(" ")
                .map(|i| i.parse::<u16>().unwrap_or(0))
                .filter(|&i| i != 0)
                .collect::<Vec<u16>>()
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
            )
        })
        .collect::<Vec<_>>();
    dbg!(sum.clone());
    dbg!(sum.iter().fold(1, |x, &y| (x as u32) * (y as u32)));
}

fn value(time: u16, print: Blueprint, mut res: Res, bots: Bots) -> u16 {
    if time <= 0 {
        return res.geo;
    }
    let mut g = 0;
    let mut build = vec![
        (res.ore >= print.ore),
        (res.ore >= print.clay),
        (res.ore >= print.obsi.0) & (res.clay >= print.obsi.1),
        (res.ore >= print.geo.0) & (res.obsi >= print.geo.1),
    ];

    if build[3] | (bots.ore >= print.clay) | (time * print.clay <= bots.ore * time + res.ore) {
        build[0] = false;
    }
    if build[3] | (bots.clay >= print.obsi.1) | (time * print.obsi.1 <= bots.clay * time + res.ore)
    {
        build[1] = false;
    }
    if build[3] | (bots.obsi >= print.geo.1) | (time * print.clay <= bots.ore * time + res.ore) {
        build[2] = false;
    }
    if time < 2 {
        build[3] = false;
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
