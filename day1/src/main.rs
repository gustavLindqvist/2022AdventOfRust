use std::collections::BinaryHeap;
fn main() {
    vec![1,3].iter().for_each(|n| println!("{}",include_str!("source.txt").split("\n\n").map(|s|{s.split('\n').map(|i|i.parse::<i32>().unwrap()).sum()}).collect::<BinaryHeap<i32>>().iter().take(*n).sum::<i32>()));
}