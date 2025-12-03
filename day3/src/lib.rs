pub fn solve_part_1(input: &String) -> u32
{
    return input.lines()
        .map(|line| line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>())
        .map(|bv| {
            let (first_ind, first_val) = match (0..(bv.len() - 1))
                .map(|i| (i, bv[i]))
                .max_by(|(i1, v1), (i2, v2)| v1.cmp(v2).then(i2.cmp(i1))) {
                Some(res) => res,
                _ => {return 0} 
            };

            return (first_val * 10) + match ((first_ind + 1)..bv.len())
                .map(|i| bv[i])
                .max() {
                Some(res) => res,
                _ => {return 0} 
            };
        })
        .sum();
}

pub fn solve_part_2(input: &String) -> u64
{
    return input.lines()
        .map(|line| line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>())
        .map(|bv| {
            let num_len = 12;
            let mut res = 0;
            let mut start = 0;
            let mut end = bv.len() - (num_len - 1);
            
            for _ in 0..num_len {
                let (ind, val) = match (start..end)
                    .map(|i| (i, bv[i]))
                    .max_by(|(i1, v1), (i2, v2)| v1.cmp(v2).then(i2.cmp(i1))) {
                    Some(res) => res,
                    _ => {return 0} 
                };

                res = (res * 10) + val as u64;
                start = ind + 1;
                end += 1;
            }

            return res;
        })
        .sum();
}