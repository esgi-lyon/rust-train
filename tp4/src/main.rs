use core::ops::Range;
use std::{env, io::stdin};

fn init_mem(range: Range<i64>) -> Vec<i64> {
    return range.map(|_| 0).collect();
}

fn case_move(mem: Vec<i64>, current: i64, the_move: i64) -> (Vec<i64>, i64) {
    let new_curr = if (current + the_move) < 0 || current == 1000 {
        current
    } else {
        current + the_move
    };

    return (mem, new_curr);
}

fn move_case_left(mem: Vec<i64>, current: i64,) -> (Vec<i64>, i64) { case_move(mem, current, -1) }

fn move_case_right(mem: Vec<i64>, current: i64,) -> (Vec<i64>, i64) { case_move(mem, current, 1) }

fn add_to_case(mut mem: Vec<i64>, current: i64, to_add: i64) -> (Vec<i64>, i64) {
    mem[current as usize] += to_add;

    return (mem, current);
}

fn increment_case(mem: Vec<i64>, current: i64) -> (Vec<i64>, i64) {
    return add_to_case(mem, current, 1);
}

fn decrement_case(mem: Vec<i64>, current: i64) -> (Vec<i64>, i64) {
    return add_to_case(mem, current, -1);
}

fn show_case(mem: Vec<i64>, current: i64) -> (Vec<i64>, i64) {
    print!("[{}]: {}\n", current, mem[current as usize]);
    return (mem, current);
}

fn input_to_case(mut mem: Vec<i64>, current: i64) -> (Vec<i64>, i64) {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    println!("{}", input);
    let store: i64 = mem[current as usize];
    mem[current as usize] = input.parse::<i64>().unwrap_or(store);

    return (mem, current);
}

fn start_while(
    mut in_while: Vec<fn(Vec<i64>, i64) -> (Vec<i64>, i64)>,
    mem: Vec<i64>,
    current: i64,
) -> (Vec<i64>, i64, Vec<fn(Vec<i64>, i64) -> (Vec<i64>, i64)>) {
    in_while.push(|mem: Vec<i64>, current: i64| (mem, current));

    return (mem, current, in_while);
}

fn add_to_while_if_not_empty(
    mut mem: Vec<i64>,
    mut current: i64,
    mut cmds: Vec<fn(Vec<i64>, i64) -> (Vec<i64>, i64)>,
    current_cmd: fn(Vec<i64>, i64) -> (Vec<i64>, i64),
) -> (Vec<i64>, i64, Vec<fn(Vec<i64>, i64) -> (Vec<i64>, i64)>) {
    if cmds.len() < 1 {
        (mem, current) = current_cmd(mem, current);
        return (mem, current, cmds);
    }

    cmds.push(current_cmd);

    return (mem, current, cmds);
}

fn process_while(
    mut in_while: Vec<fn(Vec<i64>, i64) -> (Vec<i64>, i64)>,
    mem: Vec<i64>,
    current: i64,
) -> (Vec<i64>, i64, Vec<fn(Vec<i64>, i64) -> (Vec<i64>, i64)>) {
    let mut res_mem: Vec<i64> = mem.clone();
    
    while let Some(cmd) = in_while.iter().next() {
        (res_mem, _) = cmd(res_mem, current);
    }

    in_while.clear();

    return (res_mem, current, in_while);
}

fn main() {
    let mut mem: Vec<i64> = init_mem(0..1000);
    let args: Vec<String> = env::args().collect();
    let mut current: i64 = 0;
    let mut in_while: Vec<fn(Vec<i64>, i64) -> (Vec<i64>, i64)> = vec![];

    if let Some((_, brainfuck_cmds)) = args.split_first() {
        for arg in brainfuck_cmds {
            (mem, current, in_while) = match arg.as_str() {
                "+" => add_to_while_if_not_empty(mem, current, in_while, increment_case),
                "-" => add_to_while_if_not_empty(mem, current, in_while, decrement_case),
                ">" => add_to_while_if_not_empty(mem, current, in_while, move_case_right),
                "<" => add_to_while_if_not_empty(mem, current, in_while, move_case_left),
                "." => add_to_while_if_not_empty(mem, current, in_while, show_case),
                "," => add_to_while_if_not_empty(mem, current, in_while, input_to_case),
                "[" => start_while(in_while, mem, current),
                "]" => process_while(in_while, mem, current),
                _ => (mem, current, in_while),
            };
        }
    }
}
