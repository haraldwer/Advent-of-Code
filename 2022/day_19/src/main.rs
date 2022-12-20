use std::fs;

#[derive(Clone)]
#[derive(Copy)]
struct Blueprint {
    ore_bot_price: i32,
    clay_bot_price: i32,
    obsidian_bot_price_ore: i32,
    obsidian_bot_price_clay: i32,
    geode_bot_price_ore: i32,
    geode_bot_price_obsidian: i32
}

#[derive(Clone)]
#[derive(Copy)]
struct State {
    ore: i32,
    clay: i32, 
    obsidian: i32,
    geode: i32,

    ore_bots: i32,
    clay_bots: i32, 
    obsidian_bots: i32,
    geode_bots: i32
}

fn test_bp(bp: Blueprint, min: i32) -> State {
    if min == 0 {
        return State { 
            ore: 0, 
            clay: 0, 
            obsidian: 0, 
            geode: 0,
            ore_bots: 1,
            clay_bots: 0,
            obsidian_bots: 0,
            geode_bots: 0
        };
    }

    let mut state = test_bp(bp, min - 1);

    state.ore += state.ore_bots;
    state.clay += state.clay_bots;
    state.obsidian += state.obsidian_bots;
    state.geode += state.geode_bots;

    // branch depending on available builds
    // bigger than due to order of actions
    if state.ore > bp.ore_bot_price {
        // branch? 
    }
    if state.clay > bp.clay_bot_price {
        // branch? 
    }
    if state.ore > bp.obsidian_bot_price_ore &&
        state.clay > bp.obsidian_bot_price_clay {
        // branch? 
    }
    if state.ore > bp.geode_bot_price_ore &&
        state.obsidian > bp.geode_bot_price_obsidian {
        // branch? 
    }

    return state;
}

fn main() {

    let input = fs::read_to_string("input_example.txt").expect("failed to read file");

    for blueprint in input.lines() {

        let split = blueprint.split(" ").collect::<Vec<&str>>();

        let bp = Blueprint {
            ore_bot_price: split[6].parse::<i32>().unwrap(),
            clay_bot_price: split[12].parse::<i32>().unwrap(),
            obsidian_bot_price_ore: split[18].parse::<i32>().unwrap(),
            obsidian_bot_price_clay: split[21].parse::<i32>().unwrap(),
            geode_bot_price_ore: split[27].parse::<i32>().unwrap(),
            geode_bot_price_obsidian: split[30].parse::<i32>().unwrap()
        };

        let state = test_bp(bp, 24); 
        println!("{:?}", state.geode);
    }
}