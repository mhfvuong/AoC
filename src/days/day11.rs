use num_bigint::{BigUint, ToBigUint};

struct Monkey {
    items: Vec<BigUint>,
    operation_type: String,
    operation_value: String,
    test: u64,
    result: [usize; 2],
    items_inspected: usize,
}

impl Monkey {
    fn new() -> Self {
        Monkey{
            items: Vec::new(),
            operation_type: String::new(),
            operation_value: String::new(),
            test: 0,
            result: [0; 2],
            items_inspected: 0
        }
    }
}

pub fn monkey_trouble(data_string: String) {
    let lines = data_string.lines();
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut current_monkey = 0;
    for line in lines {
        let information = line.trim().split(' ').collect::<Vec<&str>>();
        match information[0] {
            "Monkey" => {
                monkeys.push(Monkey::new());
                current_monkey += 1;
            },
            "Starting" => {
                let items = line.trim().split("items: ").collect::<Vec<&str>>()[1];
                for item in items.split(", ").collect::<Vec<&str>>() {
                    monkeys[current_monkey - 1].items.push(
                        item.parse::<u64>().unwrap()
                            .to_biguint().unwrap()
                    )
                }
            },
            "Operation:" => {
                monkeys[current_monkey - 1].operation_type = information[4].to_string();
                monkeys[current_monkey - 1].operation_value = information[5].to_string();
            } ,
            "Test:" => monkeys[current_monkey - 1].test =
                information[3].parse::<u64>().unwrap(),
            "If" => {
                if information[1] == "true:" { monkeys[current_monkey - 1].result[1] = information[5].parse::<usize>().unwrap();}
                else {monkeys[current_monkey - 1].result[0] = information[5].parse::<usize>().unwrap();}
            },
            _ => ()
        }
    }
    let mut lcm = 1.to_biguint().unwrap();
    for monk in &monkeys {
        lcm *= monk.test;
    }
    for _round in 0..10_000 { // part 1 uses 20
        for n in 0..monkeys.len() {
            for _i in 0..monkeys[n].items.len() {
                let mut item = monkeys[n].items[_i].clone();
                let operation_value;
                if monkeys[n].operation_value == "old" {
                    operation_value = item.clone();
                } else {
                    operation_value = monkeys[n].operation_value.parse::<u64>().unwrap().to_biguint().unwrap();
                }
                let test_result = monkeys[n].result;
                if monkeys[n].operation_type == "*" {
                    item *= operation_value.to_biguint().unwrap();
                }
                else if monkeys[n].operation_type == "+" {
                    item += operation_value.to_biguint().unwrap();
                }
                item %= lcm.clone();
                // item /= 3.to_biguint().unwrap(); // part1: uncomment this one
                if item.clone() % monkeys[n].test == 0.to_biguint().unwrap() { monkeys[test_result[1]].items.push(item); }
                else { monkeys[test_result[0]].items.push(item); }
            }
            monkeys[n].items_inspected += monkeys[n].items.len();
            monkeys[n].items.drain(..);
        }
        match _round {
            0 => monkey_items(&monkeys, _round),
            19 => monkey_items(&monkeys, _round),
            999 => monkey_items(&monkeys, _round),
            1999 => monkey_items(&monkeys, _round),
            2999 => monkey_items(&monkeys, _round),
            3999 => monkey_items(&monkeys, _round),
            4999 => monkey_items(&monkeys, _round),
            5999 => monkey_items(&monkeys, _round),
            6999 => monkey_items(&monkeys, _round),
            7999 => monkey_items(&monkeys, _round),
            8999 => monkey_items(&monkeys, _round),
            9999 => monkey_items(&monkeys, _round),
            _ => ()
        }
    }
    let mut vec = Vec::new();
    for monkey in monkeys {
        vec.push(monkey.items_inspected);
    }
    vec.sort();
    let length = vec.len();
    let monkey_business = vec.remove(length - 1) * vec.remove(length - 2);
    println!("total amount of inspections: {}", monkey_business);
}

// debugging
fn monkey_items(monk: &Vec<Monkey>, rounds: usize) {
    println!("== after round {} ==", rounds + 1);
    for monkey in monk {
        println!("{:?}", monkey.items_inspected);
    }
}