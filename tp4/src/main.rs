use core::ops::Range;
use std::{env, io::stdin};

type Memory = Vec<i64>;
type InterpreterPayload = (Memory, i64);
type InterpreterProcess = fn(InterpreterPayload) -> InterpreterPayload;

fn init_mem(range: Range<i64>) -> Vec<i64> {
    return range.map(|_| 0).collect();
}

fn parse_arg(args: Vec<String>) -> Vec<String> {
    if args.len() <= 1 { return vec![]; }
    return args[1].split("").map(|e|e.to_string()).collect::<Vec<String>>();
}

fn case_move((mem, current): InterpreterPayload, the_move: i64) -> InterpreterPayload{
    let new_curr = if (current + the_move) < 0 || current == 1000 {
        current
    } else {
        current + the_move
    };

    return (mem, new_curr);
}

fn move_case_left((mem, current): InterpreterPayload) -> InterpreterPayload { case_move((mem, current), -1) }

fn move_case_right((mem, current): InterpreterPayload) -> InterpreterPayload { case_move((mem, current), 1) }

fn add_to_case((mut mem, current): InterpreterPayload, to_add: i64) -> (Vec<i64>, i64) {
    mem[current as usize] += to_add;

    return (mem, current);
}

fn increment_case((mem, current): InterpreterPayload) -> InterpreterPayload {
    return add_to_case((mem, current), 1);
}

fn decrement_case((mem, current): InterpreterPayload) -> InterpreterPayload {
    return add_to_case((mem, current), -1);
}

fn show_case((mem, current): InterpreterPayload) -> InterpreterPayload {
    print!("[{}]: {}\n", current, mem[current as usize]);
    return (mem, current);
}

fn input_to_case((mut mem, current): InterpreterPayload) -> InterpreterPayload {
    let mut input = String::new();
    print!("Enter an int to push in case {}", current);
    stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    println!("{}", input);
    let store: i64 = mem[current as usize];
    mem[current as usize] = input.parse::<i64>().unwrap_or(store);

    return (mem, current);
}

fn start_while(
    mut in_while: Vec<InterpreterProcess>,
    (mem, current): InterpreterPayload
) -> (InterpreterPayload, Vec<InterpreterProcess>) {
    // dummy command just to init while collection
    in_while.push(|(mem, current): InterpreterPayload| (mem, current));

    return ((mem, current), in_while);
}

fn add_to_while_if_not_empty(
    (mut mem, mut current): InterpreterPayload,
    mut cmds: Vec<InterpreterProcess>,
    current_cmd: InterpreterProcess,
) -> (InterpreterPayload, Vec<InterpreterProcess>) {
    if cmds.len() < 1 {
        (mem, current) = current_cmd((mem, current));
        return ((mem, current), cmds);
    }

    cmds.push(current_cmd);

    return ((mem, current), cmds);
}

fn process_while(
    mut in_while: Vec<InterpreterProcess>,
    (mem, mut current): InterpreterPayload
) -> (InterpreterPayload, Vec<InterpreterProcess>) {
    let mut res_mem: Vec<i64> = mem.clone();
    
    if current < 0 {
        return ((res_mem, current), in_while);
    }

    while current > 0 {
        for cmd in in_while.iter() {
            (res_mem, current) = cmd((res_mem, current))
        }

        current-=1;
    }

    in_while.clear();

    return ((res_mem, current), in_while);
}

fn main() {
    let mut mem: Vec<i64> = init_mem(0..1000);

    let args: Vec<String> = parse_arg(env::args().collect());
    let mut current: i64 = 0;
    let mut in_while: Vec<InterpreterProcess> = vec![];

    if let Some((_, brainfuck_op)) = args.split_first() {
        for arg in brainfuck_op {
            ((mem, current), in_while) = match arg.as_str() {
                "+" => add_to_while_if_not_empty((mem, current), in_while, increment_case),
                "-" => add_to_while_if_not_empty((mem, current), in_while, decrement_case),
                ">" => add_to_while_if_not_empty((mem, current), in_while, move_case_right),
                "<" => add_to_while_if_not_empty((mem, current), in_while, move_case_left),
                "." => add_to_while_if_not_empty((mem, current), in_while, show_case),
                "," => add_to_while_if_not_empty((mem, current), in_while, input_to_case),
                "[" => start_while(in_while, (mem, current)),
                "]" => process_while(in_while, (mem, current)),
                _ => ((mem, current), in_while),
            };
        }
    }
}
