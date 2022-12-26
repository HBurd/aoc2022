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

fn add_rock(arena: &mut Vec<bool>, jetstream: &mut Jetstream, rock: &Vec<(usize, usize)>, width: usize)
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

    let rock_widths = [4, 3, 3, 1, 2];

    let mut next_rock = 0;
    let mut rock_cnt = 0;
    let max_rocks: u64 = 1_000_000_000_000;
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

    let mut last_height = 0;

    while rock_cnt < max_rocks
    {
        if jetstream.index == 2
        {
            println!("Rocks: {}", rock_cnt);
            println!("Height: {}", arena.len() / 7 - last_height);
            rock_cnt = 0;
            last_height = arena.len() / 7;
        }

        if rock_cnt == 1704
        {
            println!("1704 height: {}", arena.len() / 7 - last_height);
        }

        add_rock(&mut arena, &mut jetstream, &rocks[next_rock], rock_widths[next_rock]);
        next_rock = (next_rock + 1) % rocks.len();
        rock_cnt += 1;
    }

    println!("{}", arena.len() / 7);
}
