use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
enum DataType {
    Light,
    Wiring,
    Joltage
}

pub fn solve_part_1(input: &String) -> u64
{
    let machines: Vec<_> = input.lines()
        .filter_map(|line| match line.chars()
            .fold((vec!(), None::<(_, String, _)>), |(mut acc, curr_data), c| match curr_data {
                Some((data_type, mut buf, closing_char)) => {
                    if c == closing_char {
                        acc.push((data_type, buf));
                        return (acc, None);
                    }
                    buf.push(c);
                    return (acc, Some((data_type, buf, closing_char)));
                },
                None => {
                    let (data_type, closing_char) = match c {
                        '[' => (DataType::Light, ']'),
                        '(' => (DataType::Wiring, ')'),
                        '{' => (DataType::Joltage, '}'),
                        _ => {return (acc, None)}
                    };

                    return (acc, Some((data_type, String::new(), closing_char)));
                }
            }).0.into_iter()
            .fold((None, vec!(), None), |(light, mut wiring, joltage), (data_type, content)| match data_type {
                DataType::Light => {
                    if light.is_some() {
                        return (light, wiring, joltage);
                    }
                    let states: Vec<_> = content.chars()
                        .filter_map(|c| match c {
                            '.' => Some(false),
                            '#' => Some(true),
                            _ => None
                        })
                        .collect();
                    return (Some(states), wiring, joltage);
                }
                DataType::Wiring => {
                    let nums: Vec<_> = content.split(",")
                        .filter_map(|v| v.trim().parse::<usize>().ok())
                        .collect();
                    wiring.push(nums);
                    return (light, wiring, joltage);
                }
                DataType::Joltage => {
                    if joltage.is_some() {
                        return (light, wiring, joltage);
                    }
                    let nums: Vec<_> = content.split(",")
                        .filter_map(|v| v.trim().parse::<usize>().ok())
                        .collect();
                    return (light, wiring, Some(nums));
                }
            }) {
            (Some(states), wiring, Some(joltage_nums)) => Some((states, wiring, joltage_nums)),
            _ => None
        })
        .collect();

    return machines.into_iter()
        .map(|(target_states, buttons, _)| {
            let mut visited = HashSet::new();

            let mut queue = VecDeque::new();
            queue.push_back((vec![false; target_states.len()], vec!()));
            while let Some((states, buttons_pressed)) = queue.pop_front() {
                if states == target_states {
                    return buttons_pressed.len();
                }

                for wiring in buttons.iter() {
                    let mut curr_states = states.clone();
                    let mut curr_buttons_pressed = buttons_pressed.clone();
                    for &i in wiring {
                        curr_states[i] = !curr_states[i];
                    }
                    curr_buttons_pressed.push(wiring);

                    if curr_states == target_states {
                        return curr_buttons_pressed.len();
                    }

                    if !visited.contains(&curr_states) {
                        queue.push_back((curr_states.clone(), curr_buttons_pressed));
                        visited.insert(curr_states);
                    }
                }
            }
            panic!();
        })
        .map(|res| res as u64)
        .sum();
}

pub fn solve_part_2(input: &String) -> u64
{
    let machines: Vec<_> = input.lines()
        .filter_map(|line| match line.chars()
            .fold((vec!(), None::<(_, String, _)>), |(mut acc, curr_data), c| match curr_data {
                Some((data_type, mut buf, closing_char)) => {
                    if c == closing_char {
                        acc.push((data_type, buf));
                        return (acc, None);
                    }
                    buf.push(c);
                    return (acc, Some((data_type, buf, closing_char)));
                },
                None => {
                    let (data_type, closing_char) = match c {
                        '[' => (DataType::Light, ']'),
                        '(' => (DataType::Wiring, ')'),
                        '{' => (DataType::Joltage, '}'),
                        _ => {return (acc, None)}
                    };

                    return (acc, Some((data_type, String::new(), closing_char)));
                }
            }).0.into_iter()
            .fold((None, vec!(), None), |(light, mut wiring, joltage), (data_type, content)| match data_type {
                DataType::Light => {
                    if light.is_some() {
                        return (light, wiring, joltage);
                    }
                    let states: Vec<_> = content.chars()
                        .filter_map(|c| match c {
                            '.' => Some(false),
                            '#' => Some(true),
                            _ => None
                        })
                        .collect();
                    return (Some(states), wiring, joltage);
                }
                DataType::Wiring => {
                    let nums: Vec<_> = content.split(",")
                        .filter_map(|v| v.trim().parse::<usize>().ok())
                        .collect();
                    if nums.len() > 0 {
                        wiring.push(nums);
                    }
                    return (light, wiring, joltage);
                }
                DataType::Joltage => {
                    if joltage.is_some() {
                        return (light, wiring, joltage);
                    }
                    let nums: Vec<_> = content.split(",")
                        .filter_map(|v| v.trim().parse::<usize>().ok())
                        .collect();
                    return (light, wiring, Some(nums));
                }
            }) {
            (Some(states), buttons, Some(joltage_nums)) => Some((states, buttons, joltage_nums)),
            _ => None
        })
        .collect();
    
    fn solve(matrix: &mut Vec<Vec<f64>>) {
        let n = matrix.len();
        let m = matrix[0].len();
        'a: for p in 0..(n.min(m)) {
            let mut max = p;
            for i in (p+1)..n {
                if matrix[i][p].abs() > matrix[max][p].abs() {
                    max = i;
                }
            }

            let mut alt_p = p;
            while matrix[max][alt_p].abs() < 1e-8 {
                alt_p += 1;
                if alt_p >= (m - 1) {
                    continue 'a;
                }
                for i in (p+1)..n {
                    if matrix[i][alt_p].abs() > matrix[max][alt_p].abs() {
                        max = i;
                    }
                }
            }

            if p != alt_p {
                for i in 0..n {
                    matrix[i].swap(p, alt_p);
                }
            }

            matrix.swap(p, max);

            let q = p;
            for i in 0..n {
                let alpha = matrix[i][q] / matrix[p][q];
                for j in 0..m {
                    if (i != p) && (j != q) {
                        matrix[i][j] -= alpha * matrix[p][j];
                    }
                }
            }

            for i in 0..n {
                if i != p {
                    matrix[i][q] = 0.0;
                }
            }

            for j in 0..m {
                if j != q {
                    matrix[p][j] /= matrix[p][q];
                }
            }
            matrix[p][q] = 1.0;
        }
    }

    return machines.into_iter()
        .map(|(_, buttons, joltage_req)| {
            let limit = *joltage_req.iter()
                .max()
                .unwrap_or(&0);

            let mut matrix = vec![vec![0.0; buttons.len() + 1]; joltage_req.len()];
            for i in 0..joltage_req.len() {
                for j in 0..buttons.len() {
                    if buttons[j].contains(&i) {
                        matrix[i][j] = 1.0;
                    }
                }
            }
            for i in 0..joltage_req.len() {
                matrix[i][buttons.len()] = joltage_req[i] as f64;
            }

            solve(&mut matrix);

            let mut main_coefs = vec![];
            for i in 0..buttons.len().min(joltage_req.len()) {
                main_coefs.push(matrix[i][i]);
            }

            let mut free_coefs = vec![vec![]; joltage_req.len()];
            for j in 0..buttons.len().min(joltage_req.len()) {
                if matrix[j][j].abs() >= 1e-8 {
                    continue;
                }

                for i in 0..joltage_req.len() {
                    free_coefs[i].push(-matrix[i][j]);
                }
            }
            for i in 0..joltage_req.len() {
                for j in joltage_req.len()..buttons.len() {
                    free_coefs[i].push(-matrix[i][j]);
                }
            }
            let free_coef_count = free_coefs[0].len();

            let mut free_values = vec![];
            for i in 0..joltage_req.len() {
                free_values.push(matrix[i][buttons.len()]);
            }

            let mut res = None::<u64>;
            let mut free_var_values = vec![0; free_coef_count];
            'a: loop {
                'b: {
                    let mut curr_res = 0;
                    for i in 0..main_coefs.len() {
                        if main_coefs[i].abs() < 1e-8 {
                            continue;
                        }

                        let mut v = free_values[i];

                        for j in 0..free_coefs[i].len() {
                            v += free_coefs[i][j] * (free_var_values[j] as f64);
                        }

                        let vr = v.round();
                        if ((v - vr).abs() >= 1e-8) || (vr < 0.0) {
                            break 'b;
                        }

                        curr_res += vr as u64;
                    }

                    for var in free_var_values.iter() {
                        curr_res += var;
                    }

                    res = match res {
                        Some(res_val) => Some(res_val.min(curr_res)),
                        None => Some(curr_res)
                    }
                }

                for i in 0..free_var_values.len() {
                    free_var_values[i] += 1;
                    if free_var_values[i] <= (limit as u64) {continue 'a}
                    free_var_values[i] = 0;
                }
                break;
            }

            return res.expect("Input should be solvable but could not find solution");
        }).sum();
}