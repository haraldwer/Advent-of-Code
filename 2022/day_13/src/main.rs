
struct Entry {
    list: Vec<Entry>,
    value: i32
}

#[derive(PartialEq)]
enum Result {
    Correct,
    Incorrect,
    None
}

fn add_pending(pending: &mut Vec<char>, entry: &mut Entry) {
    if pending.len() > 0 {
        let string = pending.iter().collect::<String>();
        entry.list.push(Entry {
            list: Vec::new(),
            value: string.parse::<i32>().unwrap()
        });
    }
    pending.clear();
}

fn parse_list(entry: &mut Entry, input: &str, offset: usize) -> usize {
    let mut skip = offset;
    let mut pending: Vec<char> = Vec::new();
    for (i, c) in input.chars().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        match c {
            '[' => {
                entry.list.push(Entry { list: Vec::new(), value: -1 });
                skip = parse_list(entry.list.last_mut().unwrap(), input, i + 1);
            }
            ']' => {
                add_pending(&mut pending, entry);
                return i - offset + 1; 
            }
            ',' => {
                add_pending(&mut pending, entry);
            }
            _ => {
                pending.push(c);
            }
        }
    }
    return input.len();
}

fn compare(left: &Entry, right: &Entry, depth: i32) -> Result {
    if left.value == -1 && right.value == -1 {
        // Comparing lists
        for (left_val, right_val) in std::iter::zip(left.list.iter(), right.list.iter()) {
            let result = compare(left_val, right_val, depth + 1);
            if result != Result::None {
                return result; 
            }
        }
        if left.list.len() != right.list.len() {
            if left.list.len() < right.list.len() { 
                return Result::Correct;
            } else { 
                return Result::Incorrect;
            };
        }
    }
    else if left.value != -1 && right.value != -1 {
        // Comparing values
        if left.value != right.value {
            if left.value < right.value { 
                return Result::Correct;
            } else { 
                return Result::Incorrect;
            }
        }
    }
    else {
        // Comparing value to list
        if left.value == -1 {
            // left is list, right is value
            let mut temp_entry = Entry { list: Vec::new(), value: -1 };
            temp_entry.list.push(Entry { list: Vec::new(), value: right.value });
            let result = compare(left, &temp_entry, depth + 1);
            if result != Result::None {
                return result;
            }
        } else {
            // left is value, right is list
            let mut temp_entry = Entry { list: Vec::new(), value: -1 };
            temp_entry.list.push(Entry { list: Vec::new(), value: left.value });
            let result = compare(&temp_entry, right, depth + 1);
            if result != Result::None {
                return result;
            }
        }
    }
    return Result::None; 
}

fn add_divider(value: i32, entries: &mut Vec<Entry>) {
    let divider_two_value = Entry { list: Vec::new(), value: value };
    let mut divider_two_list = Entry { list: Vec::new(), value: -1 };
    let mut divider_two = Entry { list: Vec::new(), value: -1 };
    divider_two_list.list.push(divider_two_value);
    divider_two.list.push(divider_two_list);
    entries.push(divider_two);
}

fn find_divider(value: i32, entries: &Vec<Entry>) -> usize {
    return entries
        .iter()
        .position(|entry| {
            if entry.list.len() == 1 {
                if entry.list[0].list.len() == 1 {
                    return entry.list[0].list[0].value == value;
                }
            }
            return false;
        }).unwrap();
}

fn main() {

    let input = std::fs::read_to_string("input.txt").expect("failed to read file");

    // Parse
    let mut entries: Vec<Entry> = Vec::new(); 
    for line in input.lines() {
        if line.len() > 1 {
            let mut entry = Entry { list: Vec::new(), value: -1 };
            parse_list(&mut entry, line, 0); 
            entries.push(entry);
        }
    }

    // Puzzle 1 
    let mut sum = 0;
    for index in (0..entries.len()).step_by(2) {
        let result = compare(&entries[index], &entries[index + 1], 0);
        if result == Result::Correct {
            sum += (index / 2) + 1;
        }
    }
    println!("Puzzle 1 Result: {sum}");

    // Puzzle 2
    add_divider(2, &mut entries);
    add_divider(6, &mut entries);
    entries.sort_unstable_by(|first, second| {
            let result = compare(first, second, 0);
            return match result {
                Result::Correct     => { std::cmp::Ordering::Less }
                Result::None        => { std::cmp::Ordering::Equal }
                Result::Incorrect   => { std::cmp::Ordering::Greater }
            };
        });
    let first_index = find_divider(2, &entries); 
    let second_index = find_divider(6, &entries); 
    let distress_signal = (first_index + 1) * (second_index + 1);
    println!("Puzzle 2 Result: {distress_signal}");
}
