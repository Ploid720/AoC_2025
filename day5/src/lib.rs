pub fn solve_part_1(input: &String) -> u64
{
    let mut lines_it = input
        .lines()
        .map(|line| line.trim());

    let fresh_ranges: Vec<_> = lines_it
        .by_ref()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| match *line.split("-").collect::<Vec<_>>() {
            [start, end] => Some((start, end)),
            _ => None
        })
        .filter_map(|(s, e)| match (s.parse::<i64>(), e.parse::<i64>()) {
            (Ok(start), Ok(end)) => Some((start, end)),
            _ => None
        })
        .collect();

    let available: Vec<_> = lines_it
        .filter_map(|id| match id.parse::<i64>() {
            Ok(id) => Some(id),
            _ => None
        })
        .collect();

    let mut res = 0;

    for id in available {
        //Small enough to not warrant binary search
        for (start, end) in &fresh_ranges {
            if (id >= *start) && (id <= *end) {
                res += 1;
                break;
            }
        }
    }

    return res;
}

pub fn solve_part_2(input: &String) -> i64
{
    let mut fresh_ranges: Vec<_> = input
        .lines()
        .map(|line| line.trim())
        .by_ref()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| match *line.split("-").collect::<Vec<_>>() {
            [start, end] => Some((start, end)),
            _ => None
        })
        .filter_map(|(s, e)| match (s.parse::<i64>(), e.parse::<i64>()) {
            (Ok(start), Ok(end)) => Some((start, end)),
            _ => None
        })
        .collect();

    fresh_ranges.sort_by(|(s1, _), (s2, _)| s1.cmp(s2));

    let fresh_ranges = match fresh_ranges.into_iter()
        .fold((vec!(), None), |(mut acc, last), (start, end)| {
            let (last_start, last_end) = match last {
                Some((ls, le)) => (ls, le),
                _ => {return (acc, Some((start, end)))}
            };

            if (last_end + 1) < start {
                acc.push((last_start, last_end));
                return (acc, Some((start, end)));
            }
            else {return (acc, Some((start.min(last_start), end.max(last_end))))}
        }) {
        (mut acc, Some(range)) => {acc.push(range); acc},
        (acc, _) => acc
    };

    let mut res = 0;

    for (start, end) in fresh_ranges {
        res += end - start + 1;
    }

    return res;
}