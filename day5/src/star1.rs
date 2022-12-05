pub fn star1() {
    let (crates, inst) = include_str!("source.txt").split_once("\n\n").unwrap();
    let mut stacks: Vec<Vec<char>> = vec![];
    let width = (crates.find('\n').unwrap() + 1) / 4;

    for _ in 0..width {
        let v: Vec<char> = vec![];
        stacks.push(v);
    }

    //zip concat something something

    let c = crates
        .split('\n')
        .rev()
        .skip(1)
        .map(|l| l.chars().collect::<Vec<char>>());

    for l in c {
        for i in (1..l.len()).step_by(4) {
            if l[i] != ' ' {
                stacks[i / 4].push(l[i]);
                print!("{}", stacks[i / 4].last().unwrap());
            } else {
                print!(" ")
            }
        }
        println!("");
    }

    for s in inst.lines().map(|l| {
        l.replace("move ", "")
            .replace(" from ", " ")
            .replace(" to ", " ")
    }) {
        let ins: Vec<usize> = s.split(" ").map(|s| s.parse::<usize>().unwrap()).collect();
        let mut to = stacks[ins[2] - 1].clone();
        for _ in 0..ins[0] {
            to.push(stacks[ins[1] - 1].pop().unwrap());
        }
        stacks[ins[2] - 1] = to;
    }
    stacks.iter().for_each(|v| print!("{}", v.last().unwrap()));
    println!("");
}
