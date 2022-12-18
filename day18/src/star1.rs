use std::collections::HashSet;

pub fn star1() {
    let mut blocks = HashSet::new();
    include_str!("data.in").lines().map(|l| l.split(",").map(|i| i.parse::<usize>().unwrap()).collect::<Vec<_>>()).for_each(|v| {blocks.insert((v[0] + 1, v[1] + 1, v[2] + 1));});

    let mut sum = 0;
    for b in &blocks{
        for s in sides(b.0,b.1,b.2){
            if !blocks.contains(&s){
                sum += 1
            }
        }
    }

    println!("{}", sum);
}

fn sides (x: usize, y:  usize, z: usize) -> Vec<(usize,usize,usize)>{
    vec![
    (x + 1, y, z),
    (x - 1, y, z),
    (x, y + 1, z),
    (x, y - 1, z),
    (x, y, z + 1),
    (x, y, z - 1)
    ]
}