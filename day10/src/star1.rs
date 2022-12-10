pub fn star1() {
    let mut regx: i32 = 1;
    let mut history = vec![1];
    for (op, nbr) in include_str!("source.txt").lines().map(|l| l.split_once(" ").unwrap_or((l, ""))){
        if op == "noop" {
            history.push(regx);
        } else {
            history.push(regx);
            history.push(regx);
            regx += nbr.parse::<i32>().unwrap();
        }
    }
    println!("{}", history.len());
    let sum: i32 = history.iter().enumerate().map(|(i, v)| (i as i32)*v ).skip(20).step_by(40).sum();
    println!("{}", sum);
}
