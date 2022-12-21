use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
};

pub fn star1() {
    let mut waiting: HashMap<&str, (&str, &str, &str)> = HashMap::new();
    let mut done: HashMap<&str, i64> = HashMap::new();

    for m in include_str!("data.in")
        .lines()
        .map(|l| l.split(" ").collect::<Vec<_>>())
    {
        let name = m[0].trim_end_matches(":");
        if m.len() == 2 {
            let v = m[1].parse().unwrap();
            done.insert(name, v);
        } else {
            waiting.insert(name, (m[1], m[2], m[3]));
        }
    }
    let res = eval(done.borrow_mut(), waiting.borrow(), "root");
    println!("{}", res);
}

fn eval<'a>(
    done: &mut HashMap<&'a str, i64>,
    waiting: &'a HashMap<&str, (&str, &str, &str)>,
    monkey: &'a str,
) -> i64 {
    if done.contains_key(monkey) {
        return *done.get(monkey).unwrap();
    }
    let (m1, op, m2) = waiting.get(monkey).unwrap();
    let m1v = eval(done, waiting, m1);
    let m2v = eval(done, waiting, m2);

    let v = match *op {
        "+" => m1v + m2v,
        "*" => m1v * m2v,
        "/" => m1v / m2v,
        _ => m1v - m2v,
    };
    done.insert(monkey, v);
    return v;
}
