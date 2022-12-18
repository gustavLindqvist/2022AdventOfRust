use std::collections::HashSet;

pub fn star2() {

    let mut blocks = HashSet::new();
    include_str!("data.in").lines().map(|l| l.split(",").map(|i| i.parse::<usize>().unwrap()).collect::<Vec<_>>()).for_each(|v| {blocks.insert((v[0] + 1, v[1] + 1, v[2] + 1));});

    let mut surface = HashSet::new();
    let mut q = vec![(0,0,0)];

    while q.len() > 0{
        let b = q.pop().unwrap();
        
        for s in sides(b.0,b.1,b.2){
            if !surface.contains(&s) & !blocks.contains(&s){
                q.push(s);            
            }
        }
        surface.insert(b);
    }

    let mut sum = 0;
    for b in &blocks{
        for s in sides(b.0,b.1,b.2){
            if surface.contains(&s){
                sum += 1;
            }
        }
    }

    println!("{}", sum);

}

fn sides (x: usize, y:  usize, z: usize) -> Vec<(usize,usize,usize)>{
    let size = 21;

    let mut s = vec![];
    if x < size {s.push((x + 1, y, z));}
    if x > 0 {s.push((x - 1, y, z));}
    if y < size {s.push((x, y + 1, z));}
    if y > 0 {s.push((x, y - 1, z));}
    if z < size {s.push((x, y, z + 1));}
    if z > 0 {s.push((x, y, z - 1));}
    s
}