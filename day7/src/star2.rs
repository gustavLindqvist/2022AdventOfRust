use std::collections::HashMap;

pub fn star2() {
    let mut dirs: HashMap<String, Vec<(i32, String)>> = HashMap::new();
    let mut path: Vec<String> = vec![];
    for lines in include_str!("source.txt").split('$') {
        let l = lines.split('\n').next().unwrap();
        if l == " ls" {
            dirs.insert(
                path.clone().concat(),
                lines
                    .split('\n')
                    //Skip ls
                    .skip(1)
                    //remove last shoutout to dbg!
                    .filter(|&s| s != "")
                    .map(|s| s.split_once(' ').unwrap())
                    .map(|(size, name)| {
                        (size.parse::<i32>().unwrap_or(0), {
                            let mut p = path.clone();
                            p.push(String::from(name));
                            p.concat()
                        })
                    })
                    .collect::<Vec<(i32, String)>>(),
            );
        } else if l == " cd .." {
            path.pop();
        } else {
            path.push(l.replace(" cd ", ""))
        }
    }
    let min_delete = size(dirs.clone(), String::from("/")) - 40000000;
    println!(
        "{}",
        dirs.clone()
            .keys()
            .map(|k| size(dirs.clone(), k.to_string()))
            .filter(|&i| i >= min_delete)
            .min()
            .unwrap()
    );
}

fn size(dirs: HashMap<String, Vec<(i32, String)>>, key: String) -> i32 {
    let e = dirs.get(&key).unwrap();
    let i = e.iter().fold(0, |last, t| {
        last + {
            if t.0 == 0 {
                size(dirs.clone(), t.1.clone())
            } else {
                t.0
            }
        }
    });
    i
}
