use std::collections::HashSet;

pub fn solve_part_1(input: &String) -> usize
{
    let papers: HashSet<_> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .filter(|(_, c)| *c == '@')
            .map(move |(x, _)| (x as isize, y as isize)))
        .collect();

    let adj: Vec<_> = ((-1)..=1)
        .flat_map(|x| ((-1)..=1).map(move |y| (x, y)))
        .filter(|(x, y)| (*x != 0) || (*y != 0))
        .collect();

    return papers.iter()
        .filter(|(x, y)| {
            let mut nbs = 0;
            for (x_off, y_off) in adj.iter() {
                if papers.contains(&(x + x_off, y + y_off)) {
                    nbs += 1;
                }
            }
            return nbs < 4;
        })
        .count();
}

pub fn solve_part_2(input: &String) -> usize
{
    let adj: Vec<_> = ((-1)..=1)
        .flat_map(|x| ((-1)..=1).map(move |y| (x, y)))
        .filter(|(x, y)| (*x != 0) || (*y != 0))
        .collect();
    
    let mut papers: HashSet<_> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .filter(|(_, c)| *c == '@')
            .map(move |(x, _)| (x as isize, y as isize)))
        .collect();
    
    let total = papers.len();

    let mut last_count = papers.len();
    loop {
        papers = papers.iter()
            .filter(|(x, y)| {
                let mut nbs = 0;
                for (x_off, y_off) in adj.iter() {
                    if papers.contains(&(x + x_off, y + y_off)) {
                        nbs += 1;
                    }
                }
                return nbs >= 4;
            })
            .cloned()
            .collect();

        let count = papers.len();
        if count == last_count {
            return total - count;
        }
        last_count = count;
    }
}