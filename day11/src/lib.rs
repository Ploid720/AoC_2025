use std::collections::HashMap;

pub fn solve_part_1(input: &String) -> u64
{
    let data: HashMap<_, _> = input.lines()
        .filter_map(|line| match *line
            .splitn(2, ":")
            .collect::<Vec<_>>() {
            [dev, outs] => Some((dev.trim(), outs.trim())),
            _ => None
        })
        .map(|(dev, outs)| (dev, outs.split_ascii_whitespace()
            .collect::<Vec<_>>()))
        .collect();

    let mut layer = HashMap::new();
    layer.insert("you", 1);
    
    let empty_vec = vec!();

    let mut res= 0;

    while !layer.is_empty() {
        let mut next_layer = HashMap::new();
        for (target, count) in layer.into_iter()
            .flat_map(|(dev, count)| data.get(dev)
                .unwrap_or(&empty_vec)
                .iter()
                .map(move |target| (target, count)))
        {
            next_layer.insert(*target, count + next_layer.get(target).unwrap_or(&0));
        }

        layer = next_layer;
        res += layer.get("out").unwrap_or(&0);
    }

    return res;
}

pub fn solve_part_2(input: &String) -> u64
{
    let data: HashMap<_, _> = input.lines()
        .filter_map(|line| match *line
            .splitn(2, ":")
            .collect::<Vec<_>>() {
            [dev, outs] => Some((dev.trim(), outs.trim())),
            _ => None
        })
        .map(|(dev, outs)| (dev, outs.split_ascii_whitespace()
            .collect::<Vec<_>>()))
        .collect();

    let mut layer = HashMap::new();
    layer.insert(("svr", false, false), 1);
    
    let empty_vec = vec!();

    let mut res= 0;

    while !layer.is_empty() {
        let mut next_layer = HashMap::new();
        for (&target, visited_dac, visited_fft, count) in layer.into_iter()
            .flat_map(|((dev, visited_dac, visited_fft), count)| data.get(&dev)
                .unwrap_or(&empty_vec)
                .iter()
                .map(move |target| (target, visited_dac, visited_fft, count)))
        {
            let visited_dac = visited_dac || (target == "dac");
            let visited_fft = visited_fft || (target == "fft");
            let key = (target, visited_dac, visited_fft);
            next_layer.insert(key, count + next_layer.get(&key).unwrap_or(&0));
        }

        layer = next_layer;
        res += layer.get(&("out", true, true)).unwrap_or(&0);
    }

    return res;
}