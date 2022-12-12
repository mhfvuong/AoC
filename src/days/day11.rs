struct Monkey {
    items: Vec<u64>,
    operation_type: String,
    operation_value: String,
    test: u64,
    result: [usize; 2],
    items_inspected: u64,
}

impl Monkey {
    fn new() -> Self {
        Monkey{
            items: Vec::new(),
            operation_type: String::new(),
            operation_value: String::new(),
            test: 1,
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
                    // println!("{item}");
                    monkeys[current_monkey - 1].items.push(item.parse::<u64>().unwrap())
                }
            },
            "Operation:" => {
                monkeys[current_monkey - 1].operation_type = information[4].to_string();
                monkeys[current_monkey - 1].operation_value = information[5].to_string();
            } ,
            "Test:" => monkeys[current_monkey - 1].test = information[3].parse::<u64>().unwrap(),
            "If" => {
                if information[1] == "true:" { monkeys[current_monkey - 1].result[1] = information[5].parse::<usize>().unwrap();}
                else {monkeys[current_monkey - 1].result[0] = information[5].parse::<usize>().unwrap();}
            },
            _ => ()
        }
    }
    for _round in 0..20 {
        for n in 0..monkeys.len() {
            for i in 0..monkeys[n].items.len() {
                let mut item = monkeys[n].items[i];
                let operation_value;
                if monkeys[n].operation_value == "old" {
                    operation_value = item
                } else { operation_value = monkeys[n].operation_value.parse::<u64>().unwrap();}
                if monkeys[n].operation_type == "*" { item *= operation_value; }
                else if monkeys[n].operation_type == "+" {item += operation_value; }
                // item /= 3; // part1: remove this one
                let test_result = monkeys[n].result;

                // println!("throw item {}", &item);

                if item % monkeys[n].test == 0 {
                    // println!("to monkey: {}", &test_result[1]);
                    monkeys[test_result[1]].items.push(item); }
                else {
                    // println!("to monkey: {}", &test_result[0]);
                    monkeys[test_result[0]].items.push(item); }
                monkeys[n].items_inspected += 1;
            }
            monkeys[n].items.drain(..);
        }
        // println!("monkeys hold the items after round {}", _round + 1);
        // println!("Monkey 0: {:?}", &monkeys[0].items);
        // println!("Monkey 1: {:?}", &monkeys[1].items);
        // println!("Monkey 2: {:?}", &monkeys[2].items);
        // println!("Monkey 3: {:?}", &monkeys[3].items);
    }
    let mut vec = Vec::new();
    for monkey in monkeys {
        // println!("{}", &monkey.items_inspected);
        vec.push(monkey.items_inspected);
    }
    vec.sort();
    let monkey_business = vec[vec.len() - 1] * vec[vec.len() - 2];
    println!("total amount of inspections: {}", monkey_business);
}