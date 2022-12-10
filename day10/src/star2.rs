pub fn star2() {
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
    for r in 0..6{
        for c in 0..40{
            if i32::abs(history[r*40 + c + 1] - c as i32) < 2{
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}
