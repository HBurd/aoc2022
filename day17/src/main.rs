fn print_rock(rock: &Vec<(usize, usize)>)
{
    let mut idx = 0;

    for row in 0..10
    {
        for col in 0..10
        {
            if idx < rock.len() && row == rock[idx].0 && col == rock[idx].1
            {
                idx += 1;
                print!("#");
            }
            else
            {
                print!(".");
            }
        }

        println!();

        if idx >= rock.len()
        {
            return;
        }
    }
}

fn move_rock(rock: &Vec<(usize, usize)>, row_offset: &mut usize, col_offset: &mut usize, row_delta: isize, col_delta: isize, arena: &Vec<bool>) -> bool
{
    for (row, col) in rock.iter()
    {
        let new_row = (*row + *row_offset) as isize + row_delta;
        let new_col = (*col + *col_offset) as isize + col_delta;

        if new_col >= 7 || new_col < 0
        {
            return false;
        }

        let index = 7 * new_row as usize + new_col as usize;

        if index < arena.len() && arena[index]
        {
            return false;
        }
    }

    *row_offset = (*row_offset as isize + row_delta) as usize;
    *col_offset = (*col_offset as isize + col_delta) as usize;

    return true;
}

fn add_rock(arena: &mut Vec<bool>, jetstream: &mut Jetstream, rock: &Vec<(usize, usize)>)
{
    let mut col_offset = 2;
    let mut row_offset = arena.len() / 7;

    move_rock(rock, &mut row_offset, &mut col_offset, 0, jetstream.next(), arena);
    move_rock(rock, &mut row_offset, &mut col_offset, 0, jetstream.next(), arena);
    move_rock(rock, &mut row_offset, &mut col_offset, 0, jetstream.next(), arena);
    move_rock(rock, &mut row_offset, &mut col_offset, 0, jetstream.next(), arena);

    while move_rock(rock, &mut row_offset, &mut col_offset, -1, 0, arena)
    {
        move_rock(rock, &mut row_offset, &mut col_offset, 0, jetstream.next(), arena);
    }

    // Update arena
    for (row, col) in rock
    {
        let index = 7 * (row + row_offset) + col + col_offset;
        if index >= arena.len()
        {
            arena.resize(arena.len() + 7, false);
        }
        arena[index] = true;
    }
}

fn print_arena(arena: &Vec<bool>)
{
    for row in (0..(arena.len() / 7)).rev()
    {
        for col in 0..7
        {
            print!("{}",
                if arena[7 * row + col]
                {
                    '#'
                }
                else
                {
                    '.'
                }
            );
        }
        println!();
    }
}

struct Jetstream
{
    jets: Vec<isize>,
    index: usize,
}

impl Jetstream
{
    // Note that this is not an iterator implementation
    // because result isn't optional
    fn next(&mut self) -> isize
    {
        let result = self.jets[self.index];
        self.index = (self.index + 1) % self.jets.len();
        return result;
    }
}

fn main() {
    let rocks = [
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];

    for rock in rocks.iter()
    {
        print_rock(&rock);
        println!();
    }

    let mut next_rock = 0;
    let mut rocks_remaining: u64 = 1_000_000_000_000;
    let mut arena = vec![true; 7];
    arena.reserve(1024 * 1024 * 128);   // reserve 128Mb
    let mut jetstream = Jetstream {
        jets:
            include_str!("../input.txt")
            .chars()
            .filter_map(
                |jet| match jet
                {
                   '<' => Some(-1),
                   '>' => Some(1),
                   _ => None
                })
            .collect(),
        index: 0
    };

    let mut extra_height = 0;

    while rocks_remaining > 0
    {
        add_rock(&mut arena, &mut jetstream, &rocks[next_rock]);
        next_rock = (next_rock + 1) % rocks.len();
        rocks_remaining -= 1;

        if arena.len() >=  1024 * 1024 * 128
        {
            // grab the last 512 lines (arbitrary)
            let mut new_vec = Vec::new();
            new_vec.reserve(1024 * 1024 * 128);
            new_vec.resize(512 * 7, false);
            new_vec[..].clone_from_slice(&arena[arena.len() - 512 * 7..]);
            extra_height += arena.len() / 7 - 512;
            arena = new_vec;

            println!("{}", rocks_remaining);
        }
        //print_arena(&arena);
        //println!();
    }

    println!("{}", arena.len() / 7 + extra_height);
}
