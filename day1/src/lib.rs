#[derive(Debug)]
enum Direction {
    Left,
    Right
}

pub fn solve_part_1(input: &String) -> usize
{
    let rotations: Vec<_> = input.lines()
        .filter_map(|line| match line.chars().next() {
            Some('L') => line[1..].parse::<i32>().map(|dist| (Direction::Left, dist)).ok(),
            Some('R') => line[1..].parse::<i32>().map(|dist| (Direction::Right, dist)).ok(),
            _ => None
        })
        .collect();

    let mut pos = 50;
    let mut res = 0;

    for (dir, dist) in rotations {
        match dir {
            Direction::Left => pos = (pos - dist).rem_euclid(100),
            Direction::Right => pos = (pos + dist).rem_euclid(100)
        }

        if pos == 0 {
            res += 1;
        }
    }

    return res;
}

pub fn solve_part_2(input: &String) -> usize
{
    let rotations: Vec<_> = input.lines()
        .filter_map(|line| match line.chars().next() {
            Some('L') => line[1..].parse::<i32>().map(|dist| (Direction::Left, dist)).ok(),
            Some('R') => line[1..].parse::<i32>().map(|dist| (Direction::Right, dist)).ok(),
            _ => None
        })
        .collect();

    let mut pos = 50;
    let mut res = 0;

    for (dir, mut dist) in rotations {
        res += dist.abs() as usize / 100;
        dist = dist % 100;
        if dist == 0 {
            continue;
        }
        
        let last_pos = pos;
        let pos_raw = match dir {
            Direction::Left => pos - dist,
            Direction::Right => pos + dist
        };

        pos = pos_raw.rem_euclid(100);

        if (pos == 0) || ((last_pos != 0) && (pos != pos_raw)) {
            res += 1;
        }
    }

    return res;
}