use std::collections::HashSet;

pub fn star1() {
    let mut beacons: Vec<(i32, i32)> = vec![];
    let mut sensors: Vec<(i32, i32)> = vec![];

    let y_row = 2000000;

    include_str!("data.in")
        .lines()
        .map(|l| {
            l.split(" ")
                .map(|s| {
                    s.trim_matches(|c: char| ['x', 'y', '=', ',', ':'].contains(&c))
                        .parse::<i32>()
                        .unwrap_or(0)
                })
                .collect()
        })
        .for_each(|l: Vec<i32>| {
            sensors.push((l[2], l[3]));
            beacons.push((l[8], l[9]));
        });

    let mut positions: HashSet<i32> = HashSet::new();
    for i in 0..sensors.len() {
        let dist = (sensors[i].0 - beacons[i].0).abs() + (sensors[i].1 - beacons[i].1).abs();
        let x_d = dist - (y_row - sensors[i].1).abs();
        for x in (sensors[i].0 - x_d)..=(sensors[i].0 + x_d) {
            positions.insert(x);
        }
    }
    for b in beacons {
        if b.1 == y_row {
            positions.remove(&b.0);
        }
    }
    println!("{}", positions.len());
}
