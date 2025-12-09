pub fn solve_part_1(input: &String) -> u64
{
    let points: Vec<_> = input.lines()
        .map(|line| line.split(",")
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<Vec<_>>())
        .filter_map(|pos| match *pos {
            [x, y] => Some((x, y)),
            _ => None
        })
        .collect();

    return (0..points.len())
        .flat_map(|i1| ((i1 + 1)..points.len())
            .map(move |i2| (i1, i2)))
        .map(|(i1, i2)| (points[i1], points[i2]))
        .map(|((x1, y1), (x2, y2))| 
            (((x2 - x1).abs() + 1) as u64) * (((y2 - y1).abs() + 1) as u64))
        .max()
        .unwrap_or(0);
}

pub fn solve_part_2(input: &String) -> u64
{
    let mut points: Vec<_> = input.lines()
        .map(|line| line.split(",")
            .filter_map(|s| s.parse::<i64>().ok())
            .collect::<Vec<_>>())
        .filter_map(|pos| match *pos {
            [x, y] => Some((x, y)),
            _ => None
        })
        .collect();

    if points
        .windows(3)
        .filter_map(|w| match w {
            [p1, p2, p3] => Some((p1, p2, p3)),
            _ => None
        })
        .chain([
                (points.get(points.len() - 2), points.last(), points.first()),
                (points.last(), points.first(), points.get(1)),
            ].into_iter()
            .filter_map(|w| match w {
                (Some(p1), Some(p2), Some(p3)) => Some((p1, p2, p3)),
                _ => None
            }))
        .fold(0, |mut turn, ((x1, y1), (x2, y2), (x3, y3))| {
            let dx_in = (x2 - x1).signum();
            let dy_in = (y2 - y1).signum();
            let dx_out = (x3 - x2).signum();
            let dy_out = (y3 - y2).signum();

            match ((dx_in, dy_in), (dx_out, dy_out)) {
                  ((-1,  0), ( 0, -1))
                | (( 0, -1), ( 1,  0))
                | (( 1,  0), ( 0,  1))
                | (( 0,  1), (-1,  0)) => {turn += 1},

                  (( 1,  0), ( 0, -1))
                | (( 0, -1), (-1,  0))
                | ((-1,  0), ( 0,  1))
                | (( 0,  1), ( 1,  0)) => {turn -= 1}

                _ => panic!("Input data should not contain non-90-degree turns")
            }

            return turn;
        }) < 0
    {
        points.reverse();
    }

    let outside_lines: Vec<_> = points
        .windows(4)
        .filter_map(|w| match w {
            [p1, p2, p3, p4] => Some((p1, p2, p3, p4)),
            _ => None
        })
        .chain([
                (points.get(points.len() - 3), points.get(points.len() - 2), points.last(), points.first()),
                (points.get(points.len() - 2), points.last(), points.first(), points.get(1)),
                (points.last(), points.first(), points.get(1), points.get(2))
            ]
            .into_iter()
            .filter_map(|w| match w {
                (Some(p1), Some(p2), Some(p3), Some(p4)) => Some((p1, p2, p3, p4)),
                _ => None
            }))
        .map(|(&(x1, y1), &(x2, y2), &(x3, y3), &(x4, y4))| {
            let (mut x2, mut y2, mut x3, mut y3) = (x2, y2, x3, y3);

            let dx = (x3 - x2).signum();
            let dy = (y3 - y2).signum();
            let off_x = dy;
            let off_y = -dx;

            let dx_prev = (x2 - x1).signum();
            let dy_prev = (y2 - y1).signum();
            let dx_next = (x4 - x3).signum();
            let dy_next = (y4 - y3).signum();

            if (dx_prev == -off_x) && (dy_prev == -off_y) {
                x2 += dx;
                y2 += dy;
            }
            if (dx_next == off_x) && (dy_next == off_y) {
                x3 -= dx;
                y3 -= dy;
            }

            let sx = x2.min(x3) + off_x;
            let sy = y2.min(y3) + off_y;
            let ex = x2.max(x3) + off_x;
            let ey = y2.max(y3) + off_y;

            return ((sx, sy), (ex, ey));
        })
        .collect();

    let mut max_area: Option<u64> = None;

    'a: for (p1, p2) in (0..points.len())
        .flat_map(|i1| ((i1 + 1)..points.len())
            .map(move |i2| (i1, i2)))
        .map(|(i1, i2)| (points[i1], points[i2]))
    {
        let ((x1, y1), (x2, y2)) = (p1, p2);
        let sx = x1.min(x2);
        let sy = y1.min(y2);
        let ex = x1.max(x2);
        let ey = y1.max(y2);
        for &((osx, osy), (oex, oey)) in outside_lines.iter() {
            if (ex < sx || ex > osx)
            && (ey < sy || ey > osy)
            && (oex < osx || oex > sx)
            && (oey < osy || oey > sy) {continue 'a}
        }
        let area = (((ex - sx) as u64) + 1) * (((ey - sy) as u64) + 1);
        max_area = match max_area {
            Some(a) => Some(a.max(area)),
            None => Some(area)
        }
    }

    return max_area.unwrap_or(0);
}