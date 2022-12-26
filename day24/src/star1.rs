pub fn star1() {
    let mut bliz: Vec<Vec<u128>> = vec![];
    let lines = include_str!("data.in").lines().collect::<Vec<&str>>();

    for y in 0..lines.len() as usize {
        let (mut w, mut r, mut d, mut l, mut u) = (0, 0, 0, 0, 0);
        for (x, c) in lines[y].chars().enumerate() {
            match c as char {
                '>' => r |= 1 << x,
                'v' => d |= 1 << x,
                '<' => l |= 1 << x,
                '^' => u |= 1 << x,
                '#' => w |= 1 << x,
                _ => {}
            }
        }
        bliz.push(vec![w, r, d, l, u]);
    }
    let size = (lines[0].len(), lines.len());
    let start = (1, 0);
    let end = (lines[0].len() - 2, lines.len() - 1);

    let mut posses: Vec<u128> = Vec::new();
    posses.resize(size.1, 0);
    posses[start.1] |= 1 << start.0;

    let mut time = 0;

    while ((posses[end.1] >> end.0) & 1) == 0 {
        next_bliz(&mut bliz, size);

        let mut prev = 0;
        for i in 0..bliz.len() {
            let curr = posses[i];
            let next = if i < bliz.len() - 1 { posses[i + 1] } else { 0 };

            let blocks = bliz[i][0] | bliz[i][1] | bliz[i][2] | bliz[i][3] | bliz[i][4];
            posses[i] = prev | curr | next;
            posses[i] |= curr << 1 | curr >> 1;

            posses[i] &= !blocks;
            prev = curr;
        }
        time += 1;
    }
    println!("{}", time);
}

fn next_bliz(bliz: &mut Vec<Vec<u128>>, size: (usize, usize)) {
    let rows = size.1 - 1;
    let cols = size.0 - 1;
    for i in 0..rows {
        let r = &mut bliz[i][1];
        *r <<= 1;
        *r &= !3;
        *r |= (*r >> (cols - 1)) & 2;

        let l = &mut bliz[i][3];
        *l >>= 1;
        *l |= (*l & 1) << (cols - 1);

        bliz[i][4] = bliz[i + 1][4];

        bliz[rows - i][3] = bliz[rows - i - 1][3];
    }

    bliz[1][2] = bliz[rows][2];
    bliz[rows - 1][4] = bliz[0][4];
}
