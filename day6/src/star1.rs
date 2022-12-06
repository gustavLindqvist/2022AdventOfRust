use std::collections::HashSet;
pub fn star1() {
    let inp: Vec<char> = include_str!("source.txt").chars().collect();

    let mut index = 4;
    for i in 4..inp.len() {
        let mut s: HashSet<char> = HashSet::new();
        for j in (i - 4)..i {
            s.insert(inp[j]);
        }
        if s.len() == 4 {
            println!("{index}");
            break;
        } else {
            index += 1;
        }
    }
}
