use std::collections::{HashMap};

#[derive(Debug)]
struct State< 'a>{
    time: i32,
    flow: i32,
    human: &'a str,
    ele: &'a str,
    open: HashMap<&'a str, bool>
}

pub fn star2() {
    let t: i32 = 26;
    let mut rooms = 0;
    let mut valves: HashMap<&str, (i32, Vec<&str>)> = HashMap::new();

    for l in include_str!("data.in").split("\n").map(|l| l.split(" ").map(|s| s).collect::<Vec<_>>()){
        let valve = l[1];
        let flow = l[4].trim_matches(|c| ['r','a','t','e','=',';'].contains(&c)).parse::<i32>().unwrap();
        let route = (9..l.len()).map(|i| l[i].trim_matches(',')).collect();
        if flow > 0 {
            rooms += 1
        }
        valves.insert(valve, (flow, route));
    }

    //Super hack hashset with doubles of flow and room as key
    let mut states: HashMap<(i32, &str, &str), State> = HashMap::new();
    let start = State{
        time: 0,
        flow: 0,
        human: "AA",
        ele: "AA",
        open: HashMap::new()
    };
    states.insert((0, "AA", "AA"), start);

    let mut max_flow = 0;

    'search: for minute in 0..t {

        println!("Searching... min {}, tree size {}", minute, states.len());
        println!("Flow this far: {}", max_flow);

        let mut next_states = HashMap::new();
        for (_, s) in &states {
            if s.open.len() == rooms{
                break 'search;
            }

            //8gb ram hack 
            if (minute > 12) & (s.open.len() < 5){
                continue;
            }
            let human = s.human;
            let elephant = s.ele;
            

            //Both walk
            for &h in &valves.get(human).unwrap().1 {
                for &e in &valves.get(elephant).unwrap().1 {
                    if !next_states.contains_key(&(s.flow, h, e)) & !next_states.contains_key(&(s.flow, h, e)){
                        next_states.insert((s.flow, h, e), State{
                            time: s.time + 1,
                            flow: s.flow,
                            human: h,
                            ele: e,
                            open: s.open.clone()
                        });
                    }
                }
            }

            //Elephant opens valve for every human walk
            if valves.get(elephant).unwrap().0 > 0{
                if !s.open.contains_key(elephant) {

                    let next_flow = s.flow + &valves.get(elephant).unwrap().0 * (t - minute - 1);
                    max_flow = max_flow.max(next_flow);
                    
                    let mut next_open = s.open.clone();
                    next_open.insert(elephant, true);
                    
                    for &h in &valves.get(human).unwrap().1 {
                        if !next_states.contains_key(&(next_flow, h, elephant)) & !next_states.contains_key(&(next_flow, elephant, h)){
                            next_states.insert((next_flow, h, elephant), State{
                                time: s.time + 1,
                                flow: next_flow,
                                human: h,
                                ele: elephant,
                                open: next_open.clone()
                            });
                        }
                    }
                }
            }

            
            //Human opens valve for every elephant walk
            if valves.get(human).unwrap().0 > 0{
                if !s.open.contains_key(human) {
                    
                    let next_flow = s.flow + &valves.get(human).unwrap().0 * (t - minute - 1);
                    max_flow = max_flow.max(next_flow);
                    
                    let mut next_open = s.open.clone();
                    next_open.insert(human, true);

                    for e in &valves.get(elephant).unwrap().1 {
                        if !next_states.contains_key(&(next_flow, human, e)) & !next_states.contains_key(&(next_flow, e, human)){
                            next_states.insert((next_flow, human, e), State{
                                time: s.time + 1,
                                flow: next_flow,
                                human: human, 
                                ele: e,
                                open: next_open.clone()
                            });
                        }
                    }
                }
            }



            //Both open valve
            if (elephant != human) & (valves.get(elephant).unwrap().0 > 0) & (valves.get(human).unwrap().0 > 0) {
                let mut next_flow = s.flow;
                let mut next_open = s.open.clone();

                if !s.open.contains_key(elephant) {             
                    next_flow += &valves.get(elephant).unwrap().0 * (t - minute - 1);
                    next_open.insert(elephant, true);
                }
                
                if !s.open.contains_key(human) {
                    next_flow += &valves.get(human).unwrap().0 * (t - minute - 1);
                    next_open.insert(human, true);
                }
                max_flow = max_flow.max(next_flow);
                if !next_states.contains_key(&(next_flow, human, elephant)) & !next_states.contains_key(&(next_flow, elephant, human)){
                    next_states.insert((next_flow, human, elephant), State{
                        time: s.time + 1,
                        flow: next_flow,
                        human: human, 
                        ele: elephant,
                        open: next_open
                    });
                }
            }
        }
        states = next_states;
    }
    println!("{}", max_flow);
}