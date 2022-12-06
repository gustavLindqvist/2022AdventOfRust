use std::collections::HashSet;
pub fn star2() {
    let inp: Vec<char> = include_str!("source.txt").chars().collect();

    let mut index = 14;
    for i in 14..inp.len() {
        let mut s: HashSet<char> = HashSet::new();
        for j in (i - 14)..i {
            s.insert(inp[j]);
        }
        if s.len() == 14 {
            println!("{index}");
            break;
        } else {
            index += 1;
        }
    }
}
