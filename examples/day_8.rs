type TreeMatrix = Vec<Vec<u8>>;

const INPUT: &str = include_str!("files/day_8");

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let matrix = build_matrix();

    let mut count = 0;
    for (y, row) in matrix.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if is_visible(&matrix, x, y) {
                count += 1;
            }
        }
    }

    dbg!(count);
}

fn part_two() {
    let matrix = build_matrix();
    let mut scores = vec![];

    for (y, row) in matrix.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            scores.push(scenic_score(&matrix, x, y));
        }
    }

    scores.sort();
    scores.reverse();

    dbg!(scores.first().unwrap());
}

fn is_visible(matrix: &TreeMatrix, x: usize, y: usize) -> bool {
    let height = matrix[y][x];
    let mut left_visible = true;
    let mut right_visible = true;
    let mut top_visible = true;
    let mut bot_visible = true;

    for i in 0..x {
        if matrix[y][i] >= height {
            left_visible = false;
        }
    }

    for i in x + 1..matrix[y].len() {
        if matrix[y][i] >= height {
            right_visible = false;
        }
    }

    for i in 0..y {
        if matrix[i][x] >= height {
            top_visible = false;
        }
    }

    for i in y + 1..matrix.len() {
        if matrix[i][x] >= height {
            bot_visible = false;
        }
    }

    left_visible || right_visible || top_visible || bot_visible
}

fn scenic_score(matrix: &TreeMatrix, x: usize, y: usize) -> usize {
    let mut left = 0;
    let mut right = 0;
    let mut top = 0;
    let mut bot = 0;

    for i in (0..x).rev() {
        left += 1;
        if matrix[y][x] <= matrix[y][i] {
            break;
        }
    }

    for i in x + 1..matrix[0].len() {
        right += 1;
        if matrix[y][x] <= matrix[y][i] {
            break;
        }
    }

    for i in y + 1..matrix.len() {
        bot += 1;
        if matrix[y][x] <= matrix[i][x] {
            break;
        }
    }

    for i in (0..y).rev() {
        top += 1;
        if matrix[y][x] <= matrix[i][x] {
            break;
        }
    }

    left * right * top * bot
}

fn build_matrix() -> TreeMatrix {
    INPUT
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}
