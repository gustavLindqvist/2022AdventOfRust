pub fn star2() {
    let mut v = include_str!("test.in")
        .lines()
        .map(|l| l.parse::<i128>().unwrap())
        // .map(|l| l.parse::<i128>().unwrap() * 811589153)
        .enumerate()
        .collect::<Vec<_>>();
    let mut c = v.clone();
    for t in v.clone() {
        print!("{}, ", t.1);
    }
    println!("");
    for i in 0..v.len() {
        let t = v.iter().enumerate().find(|t| t.1 .0 == i).unwrap();
        c.remove(t.0);
        c.insert(new as usize % v.len(), t.1.clone());
        v = c.clone();
        for t in v.clone() {
            print!("{}, ", t.1);
        }
        println!("");
    }
    let offset = v.iter().position(|i| i.1 == 0).unwrap() as usize;

    dbg!(
        v[(1000 + offset) % v.len()].1
            + v[(2000 + offset) % v.len()].1
            + v[(3000 + offset) % v.len()].1
    );
}
