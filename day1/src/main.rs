use std::fs;

fn main() {
    let contents =
        fs::read_to_string("source.txt").expect("Should have been able to read the file");

    let split = contents.split("\n\n");

    let res = split
        .map(|strings| {
            strings
                .split('\n')
                .map(|s| s.parse::<i32>().unwrap())
                .fold(0, |x, y| x + y)
        })
        .max()
        .unwrap();

    println!("{res}");
}
