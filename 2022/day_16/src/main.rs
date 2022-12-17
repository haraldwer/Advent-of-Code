use std::fs;
use std::collections::HashMap;

#[derive(Clone)]
struct Valve {
    index: u32,
    rate: u32,
    connections: Vec<String>,
    connection_indices: Vec<u32>
}

fn print_depth(depth: u32) {
    for _ in 0..(depth * 4) {
        print!(" ");
    }
}

fn test(valves: &HashMap<u32, Valve>, num_valves: u32, cache: &mut HashMap<u128, u32>, tested: u128, current: &Valve, time: u32, depth: u32) -> u32 { 
    if time == 0 {
        return 0; 
    }

    // generate cache key
    // depends on: valve id, time and tested ids
    let cache_key: u128 = 
        tested * (num_valves as u128) * 31 * 2 + // tested mask
        (current.index as u128) * 31 * 2 + // current valve
        (time as u128) * 2; // time
    match cache.get(&cache_key) {
        Some(val) => { return *val; }
        None => { }
    }

    let mut result = 0;
    if current.rate > 0 && ((tested & (1 << current.index)) == 0) {
        let new_tested = tested | (1 << current.index);
        let current_result = (time - 1) * current.rate;
        let continue_result = test(valves, num_valves, cache, new_tested, current, time - 1, depth);
        result = current_result + continue_result;
    }

    for connection in &current.connection_indices {
        let test_result = test(valves, num_valves, cache, tested, &valves[connection], time - 1, depth + 1);
        if test_result > result {
            result = test_result;
        }
    }

    cache.insert(cache_key, result);
    return result;
}

fn main() {
    let input = fs::read_to_string("input_example.txt").expect("failed to read file");

    // parse
    let mut valves = HashMap::new();
    let mut valve_name_to_index = HashMap::new();
    let mut index: u32 = 0;
    for line in input.lines() {
        let split = line.split(' ').collect::<Vec<&str>>();
        let valve_name = split[1];
        let flow_rate = split[4]
            .replace("rate=", "")
            .replace(";", "")
            .parse::<u32>()
            .unwrap();
        let mut flow_dir = Vec::new();
        for to in split.iter().skip(9) {
            let dir_name = to.replace(",","");
            flow_dir.push(dir_name);
        }
        valves.insert(index, Valve{
            index: index,
            rate: flow_rate,
            connections: flow_dir.clone(),
            connection_indices: Vec::new()
        });
        valve_name_to_index.insert(valve_name, index);
        index += 1; 

        /*
        println!("{line}");
        print!("{valve_name} {flow_rate} ");
        for dir in flow_dir {
            print!("{dir} ");
        }
        print!("\n");
        */
    }

    // sort connections
    let unsorted = valves.clone();
    for (_, valve) in &mut valves {
        for connection in &valve.connections {
            valve.connection_indices.push(valve_name_to_index[connection.as_str()]);
        }
        valve.connection_indices.sort_by(|a, b| {
            return unsorted[b].rate.cmp(&unsorted[a].rate);
        });
    }

    let mut cache: HashMap<u128, u32> = HashMap::new();
    let result = test(&valves, index, &mut cache, 0, &valves[&0], 30, 0);
    println!("Result: {result}");

    // 2488 too high
    // 2489 too high
}