pub fn star1() {
    let mut v = include_str!("data.in")
        .lines()
        .map(|l| l.parse::<i32>().unwrap())
        .enumerate()
        .collect::<Vec<_>>();
    let mut c = v.clone();
    for i in 0..v.len() {
        let t = v.iter().enumerate().find(|t| t.1 .0 == i).unwrap();
        c.remove(t.0);
        let mut new = t.0 as i32 + t.1 .1;
        while new < 0 {
            new += v.len() as i32 - 1;
        }
        while new >= v.len() as i32 {
            new = (new % v.len() as i32) + 1;
        }
        if new == 0 {
            new = v.len() as i32 - 1;
        }
        c.insert(new as usize % v.len(), t.1.clone());
        v = c.clone();
    }
    let offset = v.iter().position(|i| i.1 == 0).unwrap() as usize;

    dbg!(
        v[(1000 + offset) % v.len()].1
            + v[(2000 + offset) % v.len()].1
            + v[(3000 + offset) % v.len()].1
    );
}
