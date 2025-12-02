use std::collections::HashSet;

pub fn solve_part_1(input: &String) -> usize
{
    fn digit_count(n: i64) -> u32 {
        return n.checked_ilog10().unwrap_or(0) + 1;
    }

    return input
        .split(",")
        .into_iter()
        .map(|s| s.trim())
        .filter_map(|range| match *range.split("-").collect::<Vec<_>>() {
            [min, max] => Some((min, max)),
            _ => None
        })
        .map(|(min, max)| {
            let (min_num, max_num) = match (min.parse::<i64>(), max.parse::<i64>()) {
                (Ok(min), Ok(max))  => (min, max),
                _ => {return 0}
            };

            let min_len = min.len();
            let max_len = max.len();
            let (min_hi, max_hi) = match (
                min[..(min_len / 2)].parse::<i64>(),
                max[..(max_len / 2)].parse::<i64>()
            ) {
                (Ok(min_hi), Ok(max_hi)) => (min_hi, max_hi),
                (_, Ok(max_hi)) => (0, max_hi),
                _ => {return 0}
            };

            let (lb, ub) = match ((min.len() & 1) == 0, (max.len() & 1) == 0) {
                (true, true) => (min_hi, max_hi),
                (true, false) => (min_hi, 10_i64.pow((max_len / 2) as u32) - 1),
                (false, true) => (10_i64.pow((min_len / 2) as u32) - 1, max_hi),
                (false, false) => {return 0}
            };

            let mut res = 0;
            for i in lb..=ub {
                let n = i * 10_i64.pow(digit_count(i)) + i;
                if (n >= min_num) && (n <= max_num) {
                    res += n;
                }
            }

            return res as usize;
        })
        .sum();
}

pub fn solve_part_2(input: &String) -> usize
{
    fn digit_count(n: i64) -> u32 {
        return n.checked_ilog10().unwrap_or(0) + 1;
    }
    
    fn repeat_num(n: i64, repeat_count: usize) -> i64 {
        let mul = 10_i64.pow(digit_count(n));
        let mut res = 0;
        for _ in 0..repeat_count {
            res *= mul;
            res += n;
        }
        return res;
    }

    return input
        .split(",")
        .into_iter()
        .map(|s| s.trim())
        .filter_map(|range| match *range.split("-").collect::<Vec<_>>() {
            [min, max] => Some((min, max)),
            _ => None
        })
        .map(|(min, max)| {
            let (min_num, max_num) = match (min.parse::<i64>(), max.parse::<i64>()) {
                (Ok(min), Ok(max))  => (min, max),
                _ => {return 0}
            };

            let max_len = max.len();
            let nums: HashSet<_> = (1..=(max_len / 2))
                .flat_map(|pat_len| (2..=(max_len / pat_len)).map(move |rep_len| (pat_len, rep_len)))
                .flat_map(|(pat_len, rep_len)| (1..(10_i64.pow(pat_len as u32)))
                    .map(move |i| repeat_num(i, rep_len))
                    .filter(|n| (*n >= min_num) && (*n <= max_num)))
                .collect();

            return nums.into_iter()
                .map(|n| n as usize)
                .sum();
        })
        .sum();
}