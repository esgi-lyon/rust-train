use core::ops::Range;
use std::{env, io::stdin, vec};

type MemoryValue = u32;
type Memory = Vec<MemoryValue>;
type InterpreterPayload = (Memory, MemoryValue);
type InterpreterProcess = fn(InterpreterPayload) -> InterpreterPayload;
type AWhile = (usize, Vec<Cmd>);

// This shit code doesn't work !
//
//
#[derive(Clone)]
enum Cmd {
    While(AWhile),
    Single(InterpreterProcess),
}

fn init_mem(range: Range<MemoryValue>) -> Memory {
    return range.map(|_| 0).collect();
}

fn parse_arg(args: Vec<String>) -> Vec<String> {
    if args.len() <= 1 {
        return vec![];
    }

    return args
        .split_at(1)
        .1
        .join("")
        .replace(&['\n', '\t', '\'', ' '], "")
        .split("")
        .map(&str::to_string)
        .collect();
}

fn case_move((mem, current): InterpreterPayload, the_move: i32) -> InterpreterPayload {
    let new_curr = if (current as i32 + the_move) <= 0 {
        0
    } else {
        current as i32 + the_move
    };

    return (mem, new_curr as u32);
}

fn move_case_left((mem, current): InterpreterPayload) -> InterpreterPayload {
    case_move((mem, current), -1)
}

fn move_case_right((mem, current): InterpreterPayload) -> InterpreterPayload {
    case_move((mem, current), 1)
}

fn add_to_case((mut mem, current): InterpreterPayload, to_add: i32) -> InterpreterPayload {
    let res = (if mem[current as usize] as i32 + to_add <= 0 {
        0
    } else {
        mem[current as usize] as i32 + to_add
    }) as u32;

    mem[current as usize] = res as u32;

    return (mem, current);
}

fn increment_case((mem, current): InterpreterPayload) -> InterpreterPayload {
    return add_to_case((mem, current), 1);
}

fn decrement_case((mem, current): InterpreterPayload) -> InterpreterPayload {
    return add_to_case((mem, current), -1);
}

/// SHOW
fn show_case((mem, current): InterpreterPayload) -> InterpreterPayload {
    let the_char = char::from_u32(mem[current as usize]).unwrap_or(' ');

    print!("{}", the_char);
    return (mem, current);
}

fn input_to_case((mut mem, current): InterpreterPayload) -> InterpreterPayload {
    let mut input = String::new();
    print!("Enter an int to push in case {}", current);
    stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    println!("{}", input);
    let store: MemoryValue = mem[current as usize];
    mem[current as usize] = input.parse::<MemoryValue>().unwrap_or(store);

    return (mem, current);
}

fn close_last_while(mut cmds: Vec<Cmd>) -> Vec<Cmd> {
    if cmds.len() < 1 {
        return cmds;
    }

    let the_while = match cmds.pop().unwrap() {
        Cmd::While(v) => Option::Some(v),
        Cmd::Single(_) => Option::None,
    };

    if the_while.is_none() {
        return cmds;
    }

    let mut curr_while: AWhile = the_while.unwrap();

    if curr_while.0 > 0 {
        return cmds;
    }
    
    curr_while.0 = curr_while.1.len();

    return cmds;
}

fn add_to_commands(
    mut cmds: Vec<Cmd>,
    current_cmd: Cmd,
) -> Vec<Cmd> {
    cmds.push(current_cmd);

    return cmds;
}

// FIFO system
fn start_a_while(mut in_while: Vec<Cmd>) -> Vec<Cmd> {
    in_while.push(Cmd::While((0, vec![])));

    return in_while;
}

fn process_cmd(cmd: &Cmd, mut mem: Vec<u32>, mut current: u32) -> InterpreterPayload {
    return match cmd {
        Cmd::While((end, func)) => {
            let while_processes = Range {start:0, end: *end}
                .map(|e| {
                    return func.get(e)
                })
                .filter(|f|!f.is_none())
                .map(|f|f.unwrap());

                while_processes.for_each(|proc|{
                    (mem, current) = process_cmd(proc, mem.to_vec(), current)
                });

                return (mem, current)
        },
        Cmd::Single(proc) => proc((mem, current)),
    }
}

fn interpretor(ops: &[String]) {
    let mut mem: Memory = init_mem(0..4000);
    let mut current: u32 = 0;
    let mut command_bus: Vec<Cmd> = vec![];

    for arg in ops {
       command_bus = match arg.as_str() {
            "+" => add_to_commands( command_bus, Cmd::Single(increment_case)),
            "-" => add_to_commands(command_bus, Cmd::Single(decrement_case)),
            ">" => add_to_commands( command_bus, Cmd::Single(move_case_right)),
            "<" => add_to_commands( command_bus, Cmd::Single(move_case_left)),
            "." => add_to_commands(command_bus, Cmd::Single(show_case)),
            "," => add_to_commands( command_bus, Cmd::Single(input_to_case)),
            "[" => start_a_while(command_bus),
            "]" => close_last_while( command_bus),
            _ => command_bus,
        };
    }

    command_bus.iter().for_each(|e| {
        (mem, current) = process_cmd(e, mem.to_vec(), current)
    })
}

fn main() {
    let args: Vec<String> = parse_arg(env::args().collect());

    if let Some((_, brainfuck_ops)) = args.split_first() {
        interpretor(brainfuck_ops)
    }
}
