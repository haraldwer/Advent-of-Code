
fn main() {
    let input = std::fs::read_to_string("input.txt").expect("failed to read file");

    #[derive(Clone)]
    struct Monkey {
        operation_type: char, 
        operation_val: i32,
        test: i32,
        next_true: usize,
        next_false: usize,
        items: Vec<u128>,
        inspects: i32,
    }

    let mut monkeys = Vec::new();
    let lines = input.lines().collect::<Vec<&str>>();
    for monkey_index in 0..((lines.len() + 1) / 7)
    {
        let line_index = monkey_index * 7;
        let mut op_split = lines[line_index + 2]
            .split(' ');
        let op_type = op_split
            .nth(6)
            .unwrap()
            .parse::<char>()
            .unwrap();
        let op_str = op_split
            .nth(0)
            .unwrap();
        let op_val = match op_str {
            "old" => { -1 }
            &_ => { op_str.parse::<i32>().unwrap() }
        };
        let test = lines[line_index + 3]
            .split(' ')
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let next_true = lines[line_index + 4]
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let next_false = lines[line_index + 5]
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut items = Vec::new();
        let item_str = str::replace(lines[line_index + 1], ",", "");
        for item in item_str.split(' ').skip(4) {
            items.push(item.parse::<u128>().unwrap())
        }

        monkeys.push(Monkey {
            operation_type: op_type,
            operation_val: op_val,
            test: test,
            next_true: next_true,
            next_false: next_false,
            items: items,
            inspects: 0
        });
    }

    for round in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone(); 
            for item in monkey.items.iter() {
                let op_val = if monkey.operation_val == -1 { *item } else { monkey.operation_val as u128 };
                let worry = match monkey.operation_type {
                    '+' => { item + op_val },
                    '*' => { item * op_val }
                    _ => { 0 }
                }; 
                let relief = worry % (2 * 7 * 13 * 3 * 19 * 5 * 17 * 11);
                let to = if (relief % monkey.test as u128) == 0 { 
                    monkey.next_true 
                } else { 
                    monkey.next_false 
                };
                monkeys[to].items.push(relief); 
                monkeys[i].items.pop();
                monkeys[i].inspects += 1;
            }
        }
    }

    let mut top = 0;
    let mut second_top = 0; 
    for monkey in monkeys {
        if monkey.inspects > top {
            second_top = top;
            top = monkey.inspects;
            continue;
        }
        if monkey.inspects > second_top {
            second_top = monkey.inspects;
        }
    }
    let result = (top as u128) * (second_top as u128); 
    println!("Result: {result}");
}
