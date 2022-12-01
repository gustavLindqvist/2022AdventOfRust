use std::collections::BinaryHeap;
fn main() {
    println!("First star: {}\nSecond star: {}",include_str!("source.txt").split("\n\n").map(|s|{s.split('\n').map(|i|i.parse::<i32>().unwrap()).fold(0,|x,y|x+y)}).collect::<BinaryHeap<i32>>().iter().take(1).sum::<i32>(), include_str!("source.txt").split("\n\n").map(|s|{s.split('\n').map(|i|i.parse::<i32>().unwrap()).fold(0,|x,y|x+y)}).collect::<BinaryHeap<i32>>().iter().take(3).sum::<i32>());
}