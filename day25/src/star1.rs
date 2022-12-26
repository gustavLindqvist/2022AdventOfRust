pub fn star1() {
    println!(
        "{}",
        dec_snafu(
            include_str!("data.in")
                .lines()
                .map(|l| {
                    l.chars()
                        .collect::<Vec<_>>()
                        .iter()
                        .rev()
                        .zip(0..)
                        .map(|(c, i)| {
                            (
                                i,
                                match c {
                                    '=' => -2,
                                    '-' => -1,
                                    '0' => 0,
                                    '1' => 1,
                                    '2' => 2,
                                    _ => 404,
                                },
                            )
                        })
                        .map(|(i, c)| 5_i128.pow(i) * c)
                        .sum::<i128>()
                })
                .sum::<i128>()
        )
    );
}

fn dec_snafu(mut i: i128) -> String {
    let mut snafu = vec![];
    while i != 0 {
        let rem = i % 5;
        i = i / 5;
        match rem {
            0 => snafu.push('0'),
            1 => snafu.push('1'),
            2 => snafu.push('2'),
            3 => {
                snafu.push('=');
                i += 1;
            }
            4 => {
                snafu.push('-');
                i += 1;
            }
            _ => unreachable!(),
        };
    }
    snafu.iter().rev().collect()
}
