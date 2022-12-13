use std::ops::Add;
pub fn star2() {
    let mut result = String::from(include_str!("data.in"))
        .add("\n[[2]]\n[[6]]")
        .replace("10", "a")
        .split("\n")
        .filter(|s| s.len() != 0)
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    result.sort_by(|v1, v2| false.cmp(&valid(v1.clone(), v2.clone())));
    let r = result
        .iter()
        .map(|v| v.iter().collect::<String>())
        .collect::<Vec<_>>();
    let i1 = r.iter().position(|s| s == "[[2]]").unwrap() + 1;
    let i2 = r.iter().position(|s| s == "[[6]]").unwrap() + 1;
    println!("{}", i1 * i2);
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
