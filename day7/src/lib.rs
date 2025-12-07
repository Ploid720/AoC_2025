use std::collections::{HashMap, HashSet};

pub fn solve_part_1(input: &String) -> i64
{
    fn in_bounds(x: isize, y: isize, min_x: isize, min_y: isize, max_x: isize, max_y: isize) -> bool {
        return (x >= min_x) && (y >= min_y)
            && (x <= max_x) && (y <= max_y);
    }

    let splitters: HashSet<_> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .filter_map(move |(x, c)| match c {
                '^' => Some((x as isize, y as isize)),
                _ => None
            }))
        .collect();

    let mut beams: HashSet<_> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .filter_map(move |(x, c)| match c {
                'S' => Some((x as isize, y as isize)),
                _ => None
            }))
        .collect();

    let (min_x, min_y, max_x, max_y) = splitters
        .union(&beams)
        .fold((None, None, None, None), |(min_x, min_y, max_x, max_y), (x, y)| (
            min_x.map(|min_x: isize| min_x.min(*x)).or(Some(*x)),
            min_y.map(|min_y: isize| min_y.min(*y)).or(Some(*y)),
            max_x.map(|max_x: isize| max_x.max(*x)).or(Some(*x)),
            max_y.map(|max_y: isize| max_y.max(*y)).or(Some(*y))
        ));

    let (min_x, min_y, max_x, max_y) = (
        min_x.unwrap_or(0),
        min_y.unwrap_or(0),
        max_x.unwrap_or(0), 
        max_y.unwrap_or(0)
    );

    let mut res = 0;

    while !beams.is_empty() {
        loop {
            let mut next_beams = HashSet::new();
            let mut splitter_hit = false;
            for (x, y) in beams {
                if splitters.contains(&(x, y)) {
                    splitter_hit = true;
                    res += 1;
                    if in_bounds(x - 1, y, min_x, min_y, max_x, max_y) {
                        next_beams.insert((x - 1, y));
                    }
                    if in_bounds(x + 1, y, min_x, min_y, max_x, max_y) {
                        next_beams.insert((x + 1, y));
                    }
                }
                else {
                    next_beams.insert((x, y));
                }
            }

            beams = next_beams;
            if !splitter_hit {
                break;
            }
        }

        let mut next_beams = HashSet::new();
        for (x, mut y) in beams {
            y += 1;
            if in_bounds(x, y, min_x, min_y, max_x, max_y) {
                next_beams.insert((x, y));
            }
        }
        beams = next_beams;
    }

    return res;
}

pub fn solve_part_2(input: &String) -> u64
{
    fn in_bounds(x: isize, y: isize, min_x: isize, min_y: isize, max_x: isize, max_y: isize) -> bool {
        return (x >= min_x) && (y >= min_y)
            && (x <= max_x) && (y <= max_y);
    }

    let splitters: HashSet<_> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .filter_map(move |(x, c)| match c {
                '^' => Some((x as isize, y as isize)),
                _ => None
            }))
        .collect();

    let beams: Vec<_> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .filter_map(move |(x, c)| match c {
                'S' => Some((x as isize, y as isize)),
                _ => None
            }))
        .collect();

    let (min_x, min_y, max_x, max_y) = splitters
        .union(&beams.iter().copied().collect())
        .fold((None, None, None, None), |(min_x, min_y, max_x, max_y), (x, y)| (
            min_x.map(|min_x: isize| min_x.min(*x)).or(Some(*x)),
            min_y.map(|min_y: isize| min_y.min(*y)).or(Some(*y)),
            max_x.map(|max_x: isize| max_x.max(*x)).or(Some(*x)),
            max_y.map(|max_y: isize| max_y.max(*y)).or(Some(*y))
        ));

    let (min_x, min_y, max_x, max_y) = (
        min_x.unwrap_or(0),
        min_y.unwrap_or(0),
        max_x.unwrap_or(0), 
        max_y.unwrap_or(0)
    );

    fn solve(x: isize, y: isize, splitters: &HashSet<(isize, isize)>,
        cache: &mut HashMap<(isize, isize), u64>,
        min_x: isize, min_y: isize, max_x: isize, max_y: isize) -> u64 {
        match cache.get(&(x, y)) {
            Some(res) => {return *res},
            None => {
                if splitters.contains(&(x, y)) {
                    let res = solve(x - 1, y, splitters, cache, min_x, min_y, max_x, max_y)
                        + solve(x + 1, y, splitters, cache, min_x, min_y, max_x, max_y);
                    cache.insert((x, y), res);
                    return res;
                }
                else if !in_bounds(x, y + 1, min_x, min_y, max_x, max_y) {
                    return 1;
                }
                else {
                    return solve(x, y + 1, splitters, cache, min_x, min_y, max_x, max_y);
                }
            }
        }
    }

    return beams.into_iter()
        .map(|(x, y)| solve(x, y, &splitters, &mut HashMap::new(), min_x, min_y, max_x, max_y))
        .sum();
}