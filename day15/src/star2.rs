use std::collections::HashSet;

pub fn star2() {
    let mut beacons: Vec<(i32, i32)> = vec![];
    let mut sensors: Vec<(i32, i32)> = vec![];
    let mut dist: Vec<i32> = vec![];
    let len = 4_000_000;

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

    for i in 0..sensors.len() {
        dist.push((sensors[i].0 - beacons[i].0).abs() + (sensors[i].1 - beacons[i].1).abs());
    }

    let mut p_spots: Vec<HashSet<(i32, i32)>> = vec![];
    let toti = sensors.len() as f32;

    for i in 0..sensors.len() {
        println!("loading... {}% done", (i as f32 / toti) * 100.0);
        let mut pot = HashSet::<(i32, i32)>::new();
        let p_dist1 = dist[i] + 1;
        for y in (sensors[i].1 - p_dist1).max(0)..=(sensors[i].1 + p_dist1).min(len) {
            if (y < 0) | (y > len) {
                break;
            }
            let x1 = sensors[i].0 + (p_dist1 - (y - sensors[i].1).abs());
            if (x1 < 0) | (x1 > len) {
                break;
            }
            pot.insert((x1, y));

            let x2 = sensors[i].0 + (p_dist1 - (y - sensors[i].1).abs());
            if (x1 < 0) | (x1 > len) {
                break;
            }
            pot.insert((x2, y));
        }
        p_spots.push(pot);
    }

    for i in 0..sensors.len() {
        println!(
            "Calculating... at least {}% done",
            (i as f32 / toti) * 100.0
        );
        for j in 0..sensors.len() {
            if i != j {
                let inter = p_spots[i].intersection(&p_spots[j]);
                for (x, y) in inter {
                    let mut print = true;
                    for s in 0..sensors.len() {
                        let p_dist = dist[s];
                        if (sensors[s].0 - x).abs() + (sensors[s].1 - y).abs() <= p_dist {
                            print = false;
                        }
                    }
                    if print {
                        println!("{}, {}", x, y);
                        println!("freq: {}", *x as i128 * 4_000_000 + *y as i128);
                        return;
                    }
                }
            }
        }
    }
}
