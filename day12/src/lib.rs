use std::collections::{HashMap, HashSet};

pub fn solve(input: &String) -> u64
{
    fn parse_input(input: &String) -> (
        HashMap<i64, Vec<(i64, i64)>>,
        Vec<((i64, i64), Vec<i64>)>)
    {
        let mut shapes  = HashMap::new();
        let mut regions = vec!();
        let mut curr_ind = None;
        let mut y = 0;
        let mut curr_shape = vec!();
        let mut first_phase = true;
        'a: for line in input.lines()
            .map(|line| line.trim())
        {
            if line.is_empty() {continue}

            if first_phase {'b: {
                if line.contains(":") && !line.ends_with(":") {
                    first_phase = false;
                    break 'b;
                }

                if line.ends_with(":") {
                    if let Some(curr_ind) = curr_ind {
                        shapes.insert(curr_ind, curr_shape);
                        curr_shape = vec!();
                    }

                    curr_ind = match line[..(line.len() - 1)].parse::<i64>() {
                        Ok(ind) => Some(ind),
                        _ => {continue 'a}
                    };
                    y = 0;
                }
                else {
                    line.chars()
                        .enumerate()
                        .filter_map(|(x, c)| match c {
                            '#' => Some(x),
                            _ => None
                        })
                        .for_each(|x| {
                            curr_shape.push((x as i64, y));
                        });
                    y += 1;
                }
                continue 'a;
            }}
            
            if let Some(ci) = curr_ind {
                shapes.insert(ci, curr_shape);
                curr_shape = vec!();
                curr_ind = None;
            }

            match *line.splitn(2, ":").collect::<Vec<_>>(){
                [size_str, counts_str] => {
                    let (w, h) = match *size_str
                        .trim()
                        .splitn(2, "x")
                        .map(|n_str| n_str.parse::<i64>())
                        .collect::<Vec<_>>()
                    {
                        [Ok(w), Ok(h)] => (w, h),
                        _ => {continue 'a}
                    };

                    let counts = counts_str
                        .split_ascii_whitespace()
                        .filter_map(|n_str| n_str.parse::<i64>().ok())
                        .collect::<Vec<_>>();

                    regions.push(((w, h), counts));
                },
                _ => {}
            }
        }
        return (shapes, regions);
    }

    let (shapes, regions) = parse_input(input);

    fn get_min_max(vec: &Vec<(i64, i64)>) -> (i64, i64, i64, i64) {
        let (min_x, min_y, max_x, max_y) = vec
            .iter()
            .fold((None, None, None, None), |(min_x, min_y, max_x, max_y), (x, y)| (
                min_x.map(|min_x: i64| min_x.min(*x)).or(Some(*x)),
                min_y.map(|min_y: i64| min_y.min(*y)).or(Some(*y)),
                max_x.map(|max_x: i64| max_x.max(*x)).or(Some(*x)),
                max_y.map(|max_y: i64| max_y.max(*y)).or(Some(*y))
            ));

        return (
            min_x.unwrap_or(0),
            min_y.unwrap_or(0),
            max_x.unwrap_or(0), 
            max_y.unwrap_or(0)
        );
    }

    fn rotate_present(present: Vec<(i64, i64)>, rotation: i32) -> Vec<(i64, i64)> {
        if rotation == 0 {
            return present;
        }
        if rotation < 0 {
            unimplemented!();
        }

        let max_y = *present.iter().map(|(_, y)| y).max().unwrap_or(&0);

        let present = rotate_present(present
            .into_iter()
            .map(|(x, y)| (max_y - y, x))
            .collect(), rotation - 1);

        return present;
    }
    fn flip_present(present: Vec<(i64, i64)>, flip_ind: i32) -> Vec<(i64, i64)> {
        if flip_ind == 0 {
            return present;
        }

        let flip_v = (flip_ind & 1) != 0;
        let flip_h = (flip_ind & 2) != 0;

        let (max_x, max_y) = present
            .iter()
            .fold((None, None), |(max_x, max_y), (x, y)| (
                max_x.map(|max_x: i64| max_x.max(*x)).or(Some(*x)),
                max_y.map(|max_y: i64| max_y.max(*y)).or(Some(*y))
            ));

        let (max_x, max_y) = (
            max_x.unwrap_or(0), 
            max_y.unwrap_or(0)
        );
        
        let mut present = present;
        if flip_v {
            present = present
                .into_iter()
                .map(|(x, y)| (x, max_y - y))
                .collect();
        }
        if flip_h {
            present = present
                .into_iter()
                .map(|(x, y)| (max_x - x, y))
                .collect();
        }

        return present;
    }

    fn presents_fit(w: i64, h: i64,
        presents: &Vec<(&Vec<(Vec<(i64, i64)>, i64, i64, Vec<(i64, i64)>)>, usize)>,
        grid: &HashSet<(i64, i64)>,
        cache: &mut HashSet<Vec<(i64, i64)>>) -> bool
    {
        let mut cached_grid = grid.iter().copied().collect::<Vec<_>>();
        cached_grid.sort_by(|(x1, y1), (x2, y2)| x1.cmp(x2).then(y1.cmp(y2)));
        if cache.contains(&cached_grid) {
            return false;
        }
        cache.insert(cached_grid.clone());

        if cache.len() > 1000000 {
            cache.clear();
        }

        if presents.iter()
            .all(|(_, count)| *count == 0) {
            return true;
        }

        for i in 0..presents.len() {
            let (present_lut, count) = presents[i];
            if count <= 0 {
                continue;
            }

            let presents: Vec<_> = presents.iter()
                .enumerate()
                .filter_map(|(ind, (lut, count))| {
                    let mut count = *count;
                    if ind == i {
                        count -= 1;
                        
                    }
                    if count <= 0 {
                        return None;
                    }
                    return Some((&**lut, count));
                })
                .collect();

            'a: for (x_off, y_off, present, outline) in present_lut
                .iter()
                .flat_map(move |(present, max_x, max_y, outline)| (0..(w - max_x)).map(move |x| (x, max_y, present, outline)))
                .flat_map(|(x, max_y, present, outline)| (0..(h - max_y)).map(move |y| (x, y, present, outline)))
            {
                let present: Vec<_> = present
                    .iter()
                    .copied()
                    .map(|(x, y)| (x + x_off, y + y_off))
                    .collect();
                let mut grid = grid.clone();
                
                let grid_was_empty = grid.is_empty();

                for pos in present.iter() {
                    if (pos.0 < 0) || (pos.0 >= w)
                    || (pos.1 < 0) || (pos.1 >= h)
                    || grid.contains(&pos) {
                        continue 'a;
                    }
                }

                if !grid_was_empty {
                    if !outline.iter()
                        .map(|(x, y)| (x + x_off, y + y_off))
                        .any(|pos| grid.contains(&pos)) {
                        continue 'a;
                    }
                }
                else if !outline.iter()
                    .map(|(x, y)| (x + x_off, y + y_off))
                    .any(|(x, _)| x < 0)
                {
                    continue 'a;
                }

                for pos in present {
                    grid.insert(pos);
                }

                if presents_fit(w, h, &presents, &grid, cache) {
                    return true;
                }
            }
        }

        return false;
    }

    let mut res = 0;
    for ((w, h), counts) in regions {
        let nbs: Vec<_> = (-1..=1)
            .flat_map(|x| (-1..=1).map(move |y| (x, y)))
            .filter(|(x, y)| (*x != 0) || (*y != 0))
            .collect();

        let presents: Vec<_> = counts
            .iter()
            .enumerate()
            .map(|(i, count)| (i as i64, (*count).max(0) as usize))
            .flat_map(|(i, count)| [shapes.get(&i).unwrap()].repeat(count))
            .collect();

        if presents.iter()
            .map(|present| present.len())
            .sum::<usize>() as i64 > (w * h)
        {
            continue;
        }

        let (pw, ph) = presents.iter()
            .map(|present| get_min_max(present))
            .map(|(min_x, min_y, max_x, max_y)| (max_x - min_x + 1, max_y - min_y + 1))
            .fold((0, 0), |(mw, mh), (pw, ph)| (mw.max(pw), mh.max(ph)));

        if (((w / pw) * (h / ph)).max(0) as u64) >= (presents.len() as u64) {
            res += 1;
            continue;
        }

        //Never runs on real input but does run on test input

        let mut presents: Vec<_> = counts
            .into_iter()
            .enumerate()
            .map(|(i, count)| (i as i64, count.max(0) as usize))
            .map(|(i, count)| {
                let base_present = shapes.get(&i).unwrap();

                let present_lut: Vec<_> = (0..4)
                    .flat_map(move |rot| (0..4)
                        .map(move |flip| rotate_present(flip_present(base_present.clone(), flip), rot)))
                    .map(|mut present| {
                        let (min_x, min_y) = present
                            .iter()
                            .fold((None, None), |(min_x, min_y), (x, y)| (
                                min_x.map(|min_x: i64| min_x.min(*x)).or(Some(*x)),
                                min_y.map(|min_y: i64| min_y.min(*y)).or(Some(*y))
                            ));

                        let (min_x, min_y) = (
                            min_x.unwrap_or(0),
                            min_y.unwrap_or(0)
                        );
                        
                        for i in 0..present.len() {
                            let (x, y) = present[i];
                            present[i] = (x - min_x, y - min_y);
                        }

                        return present;
                    })
                    .map(|mut present| {
                        present.sort_by(|(x1, y1), (x2, y2)| x1.cmp(x2).then(y1.cmp(y2)));
                        return present;
                    })
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .map(|present| {
                        let (max_x, max_y) = present
                            .iter()
                            .fold((None, None), |(max_x, max_y), (x, y)| (
                                max_x.map(|max_x: i64| max_x.max(*x)).or(Some(*x)),
                                max_y.map(|max_y: i64| max_y.max(*y)).or(Some(*y))
                            ));

                        let (max_x, max_y) = (
                            max_x.unwrap_or(0), 
                            max_y.unwrap_or(0)
                        );

                        let outline: Vec<_> = present
                            .iter()
                            .flat_map(|(x, y)| nbs.iter()
                                .map(move |(x_off, y_off)| (x + x_off, y + y_off)))
                            .filter(|pos| !present.contains(pos))
                            .collect::<HashSet<_>>()
                            .into_iter()
                            .collect();
                        return (present, max_x, max_y, outline);
                    })
                    .collect();

                return (present_lut, count);
            })
            .collect();
        
        presents.sort_by(|(lut1, _), (lut2, _)| lut2.len().cmp(&lut1.len()));

        let presents_by_ref: Vec<_> = presents
            .iter()
            .map(|(lut, count)| (lut, *count))
            .collect();

        if presents_fit(w, h, &presents_by_ref, &HashSet::new(), &mut HashSet::new()) {
            res += 1;
        }
    }

    return res;
}