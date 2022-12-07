use std::collections::HashSet;

pub fn oneline() {
    //Test for dir name duplicates
    println!(
        "{}",
        include_str!("source.txt")
            .lines()
            .filter(|&s| (s != "$ cd ..") && (s != "$ ls"))
            .collect::<Vec<_>>()
            .len()
            - 2
    );
    println!(
        "{}",
        include_str!("source.txt")
            .lines()
            .collect::<HashSet<_>>()
            .len()
    );
}
