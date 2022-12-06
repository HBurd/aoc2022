type StackList = [Vec<char>; 9];
type MoveList = Vec<(u32, usize, usize)>;

fn parse_stacks<'a>(input_lines: &mut std::str::Lines<'a>) -> StackList {
    // We know there are 9 input columns
    let mut stacks: StackList = Default::default();

    loop {
        if let Some(line) = input_lines.next() {
            let line_bytes = line.as_bytes();

            // Read first two chars to see if we're done
            if line_bytes.len() >= 2 && line_bytes[1] as char == '1' {
                // Read next empty line
                input_lines.next();
                break;
            }

            for (i, stack) in stacks.iter_mut().enumerate() {
                let index = 1 + 4 * i;
                if index < line.len() && line_bytes[index] as char != ' ' {
                    stack.push(line_bytes[index] as char);
                }
            }
        }
        else {
            // Error
            break;
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    return stacks;
}

fn print_stacks(stacks: &StackList) {
    for stack in stacks.iter() {
        for c in stack.iter() {
            print!("{}", c);
        }
        print!("\n");
    }
}

fn parse_moves<'a>(input_lines: &mut std::str::Lines<'a>) -> MoveList {
    let mut moves = MoveList::new();
    loop {
        if let Some(line) = input_lines.next() {
            let mut words = line.split(' ');

            words.next();
            let count = words.next().unwrap().parse::<u32>().unwrap();
            words.next();
            let start = words.next().unwrap().parse::<usize>().unwrap() - 1;
            words.next();
            let end = words.next().unwrap().parse::<usize>().unwrap() - 1;
            moves.push((count, start, end));
        }
        else {
            // Done
            break;
        }
    }

    return moves;
}

fn execute9000(stacks: &mut StackList, moves: &MoveList) {
    for mov in moves.iter() {
        for _ in 0..mov.0 {
            let value = stacks[mov.1].pop().unwrap();
            stacks[mov.2].push(value);
        }
    }
}

fn execute9001(stacks: &mut StackList, moves: &MoveList) {
    for mov in moves.iter() {
        let end = stacks[mov.2].len();
        for _ in 0..mov.0 {
            let value = stacks[mov.1].pop().unwrap();
            stacks[mov.2].insert(end, value);
        }
    }
}

fn print_tops(stacks: &StackList)
{
    for stack in stacks.iter() {
        print!("{}", stack.last().unwrap());
    }
    print!("\n");
}

fn main() {
    let input_str = include_str!("../input.txt");
    let mut input_lines = input_str.lines();

    let mut stacks = parse_stacks(&mut input_lines);
    //print_stacks(&stacks);
    
    let mut stacks2 = stacks.clone();

    let moves = parse_moves(&mut input_lines);

    execute9000(&mut stacks, &moves);
    //print_stacks(&stacks);

    print_tops(&stacks);

    execute9001(&mut stacks2, &moves);
    print_tops(&stacks2);
}
