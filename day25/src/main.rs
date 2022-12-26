fn read_digit(c: char) -> i64
{
    match c
    {
        '=' => -2,
        '-' => -1,
        '0' => 0,
        '1' => 1,
        '2' => 2,
        _ => panic!(),
    }
}

fn print_digit(value: i64)
{
    print!("{}",
        match value
        {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => 'X',
        }
    );
}

fn smallest_remainder(x: i64, y: i64) -> i64
{
    let n = (x + y / 2) / y;
    x - n * y
}

fn main()
{
    let input = include_str!("../input.txt");

    let mut sum = 0;

    for line in input.lines()
    {
        let mut pow = 1;
        for c in line.chars().rev()
        {
            if c == '\n'
            {
                continue;
            }
            sum += pow * read_digit(c);
            pow *= 5;
        }
    }

    println!("sum: {}", sum);

    while sum > 0
    {
        let rem = smallest_remainder(sum, 5);
        print_digit(rem);
        sum -= rem;
        sum /= 5;
    }
    println!();
}
