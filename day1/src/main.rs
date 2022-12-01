use std::{collections::BinaryHeap, fs};

fn main() {
    let contents =
        fs::read_to_string("source.txt").expect("Should have been able to read the file");

    let split = contents.split("\n\n");

    let res = split.map(|strings| {
        strings
            .split('\n')
            .map(|s| s.parse::<i32>().unwrap())
            .fold(0, |x, y| x + y)
    });

    let bh: BinaryHeap<i32> = res.collect();
    let max1: i32 = bh.iter().take(1).sum();
    let max3: i32 = bh.iter().take(3).sum();

    println!("max 1:{max1}");
    println!("max 3:{max3}");
}
