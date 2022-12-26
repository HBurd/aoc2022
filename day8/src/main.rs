type Grid = [[(u32, bool); 99]; 99];

fn parse_input(input_str: &str) -> Grid {
    let mut result = [[(0, false); 99]; 99];

    let mut lines = input_str.lines();
    for i in 0..99 {
        let mut chars = lines.next().unwrap().chars();
        for j in 0..99 {
            result[i][j].0 = chars.next().unwrap().to_digit(10).unwrap();
        }
    }

    return result;
}

fn calculate_scenic_score(trees: &Grid, row: usize, col: usize) -> u32 {

    let height = trees[row][col].0;

    let mut score = 1;

    let mut factor = 0;
    for y in (0..row).rev() {
        let tree = trees[y][col];

        factor += 1;
        if tree.0 >= height {
            break;
        }
    }
    score *= factor;

    factor = 0;
    for y in (row + 1)..trees[col].len() {
        let tree = trees[y][col];

        factor += 1;
        if tree.0 >= height {
            break;
        }
    }
    score *= factor;

    factor = 0;
    for x in (0..col).rev() {
        let tree = trees[row][x];

        factor += 1;
        if tree.0 >= height {
            break;
        }
    }
    score *= factor;

    factor = 0;
    for x in (col + 1)..trees[row].len() {
        let tree = trees[row][x];

        factor += 1;
        if tree.0 >= height {
            break;
        }
    }
    score *= factor;

    return score;
}

fn main() {
    let input_str = include_str!("../input.txt");
    let mut input_grid = parse_input(input_str);

    for row in input_grid.iter_mut() {
        let mut height = 0;

        for tree in row.iter_mut() {
            if tree.0 >= height {
                tree.1 = true;
                height = tree.0 + 1;
            }
        }

        height = 0;

        for tree in row.iter_mut().rev() {
            if tree.0 >= height {
                tree.1 = true;
                height = tree.0 + 1;
            }
        }
    }

    for col in 0..99 {
        let mut height = 0;

        for row in 0..99 {
            let mut tree = &mut input_grid[row][col];
            if tree.0 >= height {
                tree.1 = true;
                height = tree.0 + 1;
            }
        }

        height = 0;

        for row in (0..99).rev() {
            let mut tree = &mut input_grid[row][col];
            if tree.0 >= height {
                tree.1 = true;
                height = tree.0 + 1;
            }
        }
    }

    let mut visible_count = 0;

    for row in input_grid.iter() {
        for tree in row.iter() {
            if tree.1 {
                visible_count += 1;
            }
        }
    }

    println!("{} visible trees", visible_count);

    let mut max_score = 0;
    
    for row in 0..input_grid.len() {
        for col in 0..input_grid[row].len() {
            max_score = std::cmp::max(max_score, calculate_scenic_score(&input_grid, row, col));
        }
    }

    println!("max score of {}", max_score);
}
