pub fn star1() {
    let mut monkeys: Vec<Monkey> = vec![];
    for m in include_str!("data.in").split("\n\n"){
        let notes = m.split("\n").map(|s| s.trim().split(" ").collect::<Vec<_>>()).collect::<Vec<_>>();
        let mon = Monkey{
            items: notes.clone()[1].iter().skip(2).map(|s| s.replace(",", "").parse::<usize>().unwrap()).collect::<Vec<_>>(),
            op: (notes[2][4].chars().nth(0).unwrap(), String::from(notes[2][5])),
            test: notes[3][3].parse::<usize>().unwrap(),
            true_monkey: notes[4][5].parse::<usize>().unwrap(),
            false_monkey: notes[5][5].parse::<usize>().unwrap()
        };
        monkeys.push(mon);
    }
    let mut inspects = vec![];
    for _ in 0..monkeys.len(){
        inspects.push(0);
    }
    for _ in 0..20{
        for j in 0..monkeys.len(){
            let m = monkeys[j].clone();
            inspects[j] += m.items.len();
            for i in 0..m.items.len(){
                let mut item = m.items[i];
                if m.op.0 == '+'{
                    if m.op.1 == "old"{
                        item = item + item;
                    } else {
                        item = item + m.op.1.parse::<usize>().unwrap();
                    }
                } else {
                    if m.op.1 == "old"{
                        item = item * item;
                    } else {
                        item = item * m.op.1.parse::<usize>().unwrap();
                    }
                }
                item = item / 3;
                if item % m.test == 0{
                    monkeys[m.true_monkey].items.push(item);
                } else {
                    monkeys[m.false_monkey].items.push(item);
                }
            }
            monkeys[j].items = vec![];
        }
    }
    inspects.sort();
    let result = inspects.iter().rev().collect::<Vec<_>>();
    dbg!(result[0] * result[1]);
}

#[derive(Clone)] 
#[derive(Debug)]
struct Monkey{
    items: Vec<usize>,
    op: (char, String),
    test: usize,
    true_monkey: usize,
    false_monkey: usize
}