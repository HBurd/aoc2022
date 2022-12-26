use std::collections::HashSet;

fn find_shortest(heightmap: &mut Vec<(i64, bool)>, dims: &(usize, usize), start: &(i64, i64), end: &(i64, i64)) -> Option<u32> {
    let mut i = 0;
    let mut frontier = HashSet::new();
    frontier.insert(*start);

    'outer: loop {
        i += 1;
        let mut next_frontier: HashSet<(i64, i64)> = HashSet::new();
        let mut stuck = true;
        for (row, col) in frontier.iter() {

            let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            let pos_usize: (usize, usize) = (*row as usize, *col as usize);


            for (row_offset, col_offset) in offsets.iter() {
                let mv_result = ((row + row_offset).try_into(), (col + col_offset).try_into());
                if let (Ok(mv_row), Ok(mv_col)) = mv_result {
                    if (mv_row as i64) < (dims.0 as i64) && (mv_col as i64) < (dims.1 as i64) {
                        let mv: (usize, usize) = (mv_row, mv_col);
                        let delta = heightmap[mv.0 * dims.1 + mv.1].0 - heightmap[pos_usize.0 * dims.1 + pos_usize.1].0;
                        if !heightmap[mv.0 * dims.1 + mv.1].1 && delta <= 1 {
                            stuck = false;
                            next_frontier.insert((mv.0 as i64, mv.1 as i64));
                            heightmap[mv.0 * dims.1 + mv.1].1 = true;
                            if mv.0 == end.0 as usize && mv.1 == end.1 as usize {
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
        if (stuck) {
            return None;
        }
        frontier = next_frontier;
    }

    return Some(i);
}

fn main() {
    let input_str = include_str!("../input.txt");
    let mut input: Vec<(i64, bool)> = Vec::new();

    let dims = (41, 95);

    let mut start: (i64, i64) = (0, 0);
    let mut end: (i64, i64) = (0, 0);

    let mut mins: Vec<(i64, i64)> = Vec::new();

    for (row, line) in input_str.lines().enumerate() {
        for (col, byte) in line.bytes().enumerate() {
            input.push((if byte as char == 'S' {
                start = (row.try_into().unwrap(), col.try_into().unwrap());
                1
            }
            else if byte as char == 'E' {
                end = (row.try_into().unwrap(), col.try_into().unwrap());
                26
            }
            else {
                (byte - 96).into()
            }, false));

            if let Some((1, _)) = input.last() {
                mins.push((row as i64, col as i64));
            }
        }
    }

    println!("From start: {:?}", find_shortest(&mut input, &dims, &start, &end));
    for ref mut cell in input.iter_mut() {
        cell.1 = false;
    }
    println!("From start: {:?}", find_shortest(&mut input, &dims, &start, &end));

    let mut min = 10000;

    for (row, col) in mins.iter() {
        for ref mut cell in input.iter_mut() {
            cell.1 = false;
        }
        if let Some(value) = find_shortest(&mut input, &dims, &(*row, *col), &end) {
            if value < min {
                min = value;
            }
        }
    }


    println!("{}", min);
}
