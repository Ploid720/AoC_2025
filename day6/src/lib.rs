use std::collections::HashMap;

#[derive(Debug)]
enum Elem {
    Add,
    Mult,
    Value(i64)
}
#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mult
}

pub fn solve_part_1(input: &String) -> i64
{
    let lut: Vec<_> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.split_ascii_whitespace()
            .enumerate()
            .map(move |(x, v)| ((x, y), v)))
        .filter_map(|(pos, v)| match v {
            "+" => Some((pos, Elem::Add)),
            "*" => Some((pos, Elem::Mult)),
            _ => v.parse::<i64>()
                .map(|val| Some((pos, Elem::Value(val))))
                .unwrap_or(None)
        })
        .collect();

    let ops: Vec<_> = lut.iter()
        .filter_map(|((x, _), elem)| match elem {
            Elem::Add => Some((x, Op::Add)),
            Elem::Mult => Some((x, Op::Mult)),
            _ => None
        })
        .collect();
    let nums: HashMap<_, _> = lut.iter()
        .filter_map(|(pos, elem)| match elem {
            Elem::Value(n) => Some((pos, n)),
            _ => None
        })
        .collect();

    let mut res = 0;

    for (op_x, op) in ops {
        let mut acc = match op {
            Op::Add => 0,
            Op::Mult => 1
        };

        let max_y = nums.keys()
            .filter(|(x, _)| x == op_x)
            .map(|(_, y)| *y)
            .max()
            .unwrap_or(0);

        for y in 0..=max_y {
            let v = match nums.get(&(*op_x, y)) {
                Some(val) => **val,
                _ => {continue}
            };
            match op {
                Op::Add => {acc += v},
                Op::Mult => {acc *= v}
            }
        }

        res += acc;
    }

    return res;
}

pub fn solve_part_2(input: &String) -> u64
{
    let lut: HashMap<_, _> = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line
            .chars()
            .enumerate()
            .map(move |(x, c)| ((x, y), c)))
        .collect();

    let w = *lut.keys()
        .map(|(x, _)| x)
        .max()
        .unwrap_or(&0);
    let h = *lut.keys()
        .map(|(_, y)| y)
        .max()
        .unwrap_or(&0);

    let op_lut: Vec<_> = (0..=w)
        .map(|x| match lut.get(&(x, h)) {
            Some('+') => (x, Some(Op::Add)),
            Some('*') => (x, Some(Op::Mult)),
            _ => (x, None)
        })
        .fold((None, vec!()), |(last, mut acc), (x, op)| match (last, op) {
            (_, Some(op)) => {
                acc.push((x, x, op));
                (Some((op, x)), acc)
            },
            (Some((op, base_x)), None) => {
                acc.push((x, base_x, op));
                (Some((op, base_x)), acc)
            },
            (None, None) => (None, acc)
        }).1;

    let mut res = 0;

    let mut last_base_x = None;
    let mut acc = 0;
    for (op_x, op_base_x, op) in op_lut {
        if Some(op_base_x) != last_base_x {
            last_base_x = Some(op_base_x);
            res += acc;
            acc = match op {
                Op::Add => 0,
                Op::Mult => 1
            };
        }

        let mut num = 0;
        let mut had_digit = false;
        for y in 0..=h {
            let v = match lut.get(&(op_x, y)) {
                Some(val) => val,
                _ => {continue}
            };
            
            let val = match v.to_digit(10) {
                Some(val) => val,
                _ => {continue}
            };

            num = (num * 10) + val as u64;
            had_digit = true;
        }

        if !had_digit {continue}
        
        match op {
            Op::Add => {acc += num},
            Op::Mult => {acc *= num}
        }
    }

    res += acc;

    return res;   
}