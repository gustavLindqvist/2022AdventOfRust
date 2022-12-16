use std::collections::{HashMap};
struct State< 'a>{
    time: i32,
    flow: i32,
    room: &'a str,
    open: HashMap<&'a str, bool>
}

pub fn star1() {
    let t = 30;

    let mut valves: HashMap<&str, (i32, Vec<&str>)> = HashMap::new();

    for l in include_str!("data.in").split("\n").map(|l| l.split(" ").map(|s| s).collect::<Vec<_>>()){
        let valve = l[1];
        let flow = l[4].trim_matches(|c| ['r','a','t','e','=',';'].contains(&c)).parse::<i32>().unwrap();
        let route = (9..l.len()).map(|i| l[i].trim_matches(',')).collect();
        
        valves.insert(valve, (flow, route));
    }

    //Super hack hashset with doubles of flow and room as key
    let mut states: HashMap<(i32, &str), State> = HashMap::new();
    let start = State{
        time: 0,
        flow: 0,
        room: "AA",
        open: HashMap::new()
    };
    states.insert((0, "AA"), start);

    let mut max_flow = 0;

    for minute in 0..t {
        let mut next_states = HashMap::new();
        for (_, s) in &states {
            
            if !s.open.contains_key(s.room) {
                
                let next_flow = s.flow + &valves.get(s.room).unwrap().0 * (t - minute - 1);
                max_flow = max_flow.max(next_flow);
                
                let mut next_open = s.open.clone();
                next_open.insert(s.room, true);
                
                next_states.insert((next_flow, s.room), State{
                    time: s.time + 1,
                    flow: next_flow,
                    room: s.room,
                    open: next_open
                });
            }

            for p in &valves.get(s.room).unwrap().1 {
                next_states.insert((s.flow, *p), State{
                    time: s.time + 1,
                    flow: s.flow,
                    room: *p,
                    open: s.open.clone()
                });
            }
        }
        states = next_states;
    }
    println!("{}", max_flow);
}