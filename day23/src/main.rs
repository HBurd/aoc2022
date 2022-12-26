use std::ops::Add;
use std::cmp;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Move
{
    NW = 0,
    N = 1,
    NE = 2,
    E = 3,
    SE = 4,
    S = 5,
    SW = 6,
    W = 7,

    Count = 8,  // Also "invalid"
    Stay = 9,
}

impl From<i32> for Move
{
    fn from(value: i32) -> Self
    {
        let cnt = Move::Count as i32;
        let value_mod = (value % cnt + cnt) % cnt;
        match value_mod
        {
            0 => Move::NW,
            1 => Move::N,
            2 => Move::NE,
            3 => Move::E,
            4 => Move::SE,
            5 => Move::S,
            6 => Move::SW,
            7 => Move::W,
            _ => Move::Stay,
        }
    }
}

impl Add<i32> for Move
{
    type Output = Self;

    fn add(self, rhs: i32) -> Move
    {
        ((self as i32) + rhs).into()
    }
}

impl Move
{
    fn apply_to(self, coord: (usize, usize)) -> (usize, usize)
    {
        let dx: isize = match self
        {
            Move::NW => -1,
            Move::W => -1,
            Move::SW => -1,
            Move::NE => 1,
            Move::E => 1,
            Move::SE => 1,
            _ => 0,
        };

        let dy: isize = match self
        {
            Move::NW => -1,
            Move::N => -1,
            Move::NE => -1,
            Move::SW => 1,
            Move::S => 1,
            Move::SE => 1,
            _ => 0,
        };

        return ((coord.0 as isize + dy) as usize,
                (coord.1 as isize + dx) as usize);
    }
}

fn main()
{
    let input = include_str!("../input.txt");

    let mut grids: [[[Option<Move>; 1024]; 1024]; 2] = [[[None; 1024]; 1024]; 2];

    for (row, line) in input.lines().enumerate()
    {
        for (col, character) in line.chars().enumerate()
        {
            if character == '#'
            {
                grids[0][row + 512][col + 512] = Some(Move::Stay);
            }
        }
    }

    let move_order = [Move::N, Move::S, Move::W, Move::E];

    let mut done = false;
    let mut default = 0;
    let mut grid_index = 0;

    let mut round = 0;
    while !done
    {
        round += 1;
        done = true;
        for row in 0..grids[grid_index].len()
        {
            for col in 0..grids[grid_index][row].len()
            {
                if grids[grid_index][row][col].is_some()
                {
                    let mut should_move = false;
                    for i in 0..(Move::Count as i32)
                    {
                        let adj = (Move::N + i).apply_to((row, col));
                        if let Some(_) = grids[grid_index][adj.0][adj.1]
                        {
                            done = false;
                            should_move = true;
                            break;
                        }
                    }

                    let mut moved = false;

                    if should_move
                    {
                        for i in 0..4
                        {
                            let mv = move_order[(default + i) % 4];
                            let adj = mv.apply_to((row, col));
                            let diag0 = (mv + 1).apply_to((row, col));
                            let diag1 = (mv + (-1)).apply_to((row, col));
                            if grids[grid_index][adj.0][adj.1].is_none()
                                && grids[grid_index][diag0.0][diag0.1].is_none()
                                && grids[grid_index][diag1.0][diag1.1].is_none()
                            {
                                if grids[(grid_index + 1) % 2][adj.0][adj.1].is_none()
                                {
                                    grids[(grid_index + 1) % 2][adj.0][adj.1] = Some(mv);
                                    moved = true;
                                }
                                else if let Some(other_mv) = grids[(grid_index + 1) % 2][adj.0][adj.1]
                                {
                                    if Move::Stay == other_mv
                                    {
                                        panic!();
                                    }
                                    else if Move::Count != other_mv
                                    {
                                        // undo other_mv
                                        grids[(grid_index + 1) % 2][adj.0][adj.1] = Some(Move::Count);
                                        
                                        let other_coord = (other_mv + 4).apply_to(adj);
                                        if grids[(grid_index + 1) % 2][other_coord.0][other_coord.1].is_some()
                                        {
                                            panic!();
                                        }

                                        grids[(grid_index + 1) % 2][other_coord.0][other_coord.1] = Some(Move::Stay);
                                    }
                                }
                                else
                                {
                                    panic!();
                                }
                                break;
                            }
                        }

                    }

                    if !moved
                    {
                        grids[(grid_index + 1) % 2][row][col] = Some(Move::Stay);
                    }
                }
            }
        }

        // Clear old grid
        for row in grids[grid_index].iter_mut()
        {
            for el in row.iter_mut()
            {
                *el = None;
            }
        }

        let mut elf_cnt = 0;

        // Clean up new grid
        for row in grids[(grid_index + 1) % 2].iter_mut()
        {
            for el in row.iter_mut()
            {
                if *el == Some(Move::Count)
                {
                    *el = None;
                }
                else if el.is_some()
                {
                    elf_cnt += 1;
                }
            }
        }

        default = (default + 1) % 4;

        grid_index = (grid_index + 1) % 2;
    }

    //for row in grids[grid_index].iter()
    //{
    //    for el in row.iter()
    //    {
    //        print!("{}", if el.is_none() { "." } else { "#" });
    //    }
    //    println!("");
    //}

    let mut min_row = 10000;
    let mut max_row = 0;

    let mut min_col = 10000;
    let mut max_col = 0;

    let mut elf_cnt = 0;

    for row in 0..grids[grid_index].len()
    {
        for col in 0..grids[grid_index][row].len()
        {
            if grids[grid_index][row][col].is_some()
            {
                elf_cnt += 1;
                min_row = cmp::min(row, min_row);
                max_row = cmp::max(row, max_row);

                min_col = cmp::min(col, min_col);
                max_col = cmp::max(col, max_col);
            }
        }
    }

    println!("{} elves, {} empty", elf_cnt, (max_row - min_row + 1) * (max_col - min_col + 1) - elf_cnt);
    println!("{}", round);
}
