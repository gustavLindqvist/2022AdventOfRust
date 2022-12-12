use std::collections::HashMap;
pub fn star2() {
    let mut map: Vec<Vec<u8>> = include_str!("data.in")
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect();

    let height = map.len();
    let width = map[0].len();
    let mut queue = vec![];
    let mut visited: HashMap<(usize, usize), i32> = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == 'S' as u8 {
                map[y][x] = 'a' as u8;
            }
            if map[y][x] == 'E' as u8 {
                queue.push((y, x));
                visited.insert((y, x), 0);
                map[y][x] = 'z' as u8;
            }
        }
    }
    while queue.len() > 0 {
        let (y, x) = queue.first().unwrap().clone();
        queue.remove(0);
        let dist = visited.get(&(y, x)).unwrap().clone();
        if map[y][x] == 'a' as u8 {
            println!("{}", dist);
            break;
        }
        if y > 0 {
            if !visited.contains_key(&(y - 1, x)) & (map[y][x] - 1 <= map[y - 1][x]) {
                visited.insert((y - 1, x), dist + 1);
                queue.push((y - 1, x));
            }
        }
        if x > 0 {
            if !visited.contains_key(&(y, x - 1)) & (map[y][x] - 1 <= map[y][x - 1]) {
                visited.insert((y, x - 1), dist + 1);
                queue.push((y, x - 1));
            }
        }
        if y < height - 1 {
            if !visited.contains_key(&(y + 1, x)) & (map[y][x] - 1 <= map[y + 1][x]) {
                visited.insert((y + 1, x), dist + 1);
                queue.push((y + 1, x));
            }
        }
        if x < width - 1 {
            if !visited.contains_key(&(y, x + 1)) & (map[y][x] - 1 <= map[y][x + 1]) {
                visited.insert((y, x + 1), dist + 1);
                queue.push((y, x + 1));
            }
        }
    }
}
