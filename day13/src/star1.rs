pub fn star1() {
    let result = include_str!("data.in")
        .replace("10", "a")
        .split("\n\n")
        .map(|s| s.split_once("\n").unwrap())
        .map(|s| {
            (
                s.0.chars().collect::<Vec<char>>(),
                s.1.chars().collect::<Vec<char>>(),
            )
        })
        .enumerate()
        .filter(|(_, t)| valid(t.0.clone(), t.1.clone()))
        .fold(0, |i, t| i + t.0 + 1);
    println!("{}", result);
}

fn valid(mut s1: Vec<char>, mut s2: Vec<char>) -> bool {
    let mut res = true;
    for i in 0..s1.len() {
        if !(s1[i] == s2[i]) {
            match (s1[i], s2[i]) {
                (_, ']') => {
                    res = false;
                    break;
                }
                (']', _) => {
                    break;
                }
                (_, '[') => {
                    s1.insert(i, '[');
                    s1.insert(i + 2, ']');
                }
                ('[', _) => {
                    s2.insert(i, '[');
                    s2.insert(i + 2, ']');
                }
                (c1, c2) => {
                    res = c1.to_digit(11).unwrap() < c2.to_digit(11).unwrap();
                    break;
                }
            };
        }
    }
    res
}
