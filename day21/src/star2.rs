use std::{
    borrow::{Borrow, BorrowMut},
    collections::HashMap,
};

pub fn star2() {
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

    let root = waiting.get("root").unwrap();
    let res1 = eval(done.borrow_mut(), waiting.borrow(), root.0);
    let res2 = eval(done.borrow_mut(), waiting.borrow(), root.2);
    if res1 == None {
        eval_expect(done.borrow_mut(), waiting.borrow(), root.0, res2.unwrap());
    } else {
        eval_expect(done.borrow_mut(), waiting.borrow(), root.2, res1.unwrap());
    }
    println!("SCREAM!: {}", done.get("humn").unwrap());
}

fn eval_expect<'a>(
    done: &mut HashMap<&'a str, i64>,
    waiting: &'a HashMap<&str, (&str, &str, &str)>,
    monkey: &'a str,
    expect: i64,
) {
    if monkey == "humn" {
        done.insert("humn", expect);
        return;
    }
    if done.contains_key(monkey) {
        return;
    }
    let (m1, op, m2) = waiting.get(monkey).unwrap();
    let m1v = eval(done, waiting, m1);
    let m2v = eval(done, waiting, m2);
    if m1v == None {
        let op2 = m2v.unwrap();
        let v = match *op {
            "+" => expect - op2,
            "*" => expect / op2,
            "/" => expect * op2,
            _ => expect + op2,
        };
        eval_expect(done, waiting, m1, v);
    }
    if m2v == None {
        let op1 = m1v.unwrap();
        let v = match *op {
            "+" => expect - op1,
            "*" => expect / op1,
            "/" => op1 / expect,
            _ => op1 - expect,
        };
        eval_expect(done, waiting, m2, v);
    }
}

fn eval<'a>(
    done: &mut HashMap<&'a str, i64>,
    waiting: &'a HashMap<&str, (&str, &str, &str)>,
    monkey: &'a str,
) -> Option<i64> {
    if monkey == "humn" {
        return None;
    }
    if done.contains_key(monkey) {
        return Some(*done.get(monkey).unwrap());
    }
    let (m1, op, m2) = waiting.get(monkey).unwrap();
    let m1v = eval(done, waiting, m1);
    let m2v = eval(done, waiting, m2);
    if (m1v == None) | (m2v == None) {
        return None;
    }
    let op1 = m1v.unwrap();
    let op2 = m2v.unwrap();
    let v = match *op {
        "+" => op1 + op2,
        "*" => op1 * op2,
        "/" => op1 / op2,
        _ => op1 - op2,
    };
    done.insert(monkey, v);
    return Some(v);
}
