use std::{fs, hint::black_box, time::{Duration, Instant}};

macro_rules! day {
    ($task:expr, $path:expr, $f1:expr, $f2:expr) => {
        Parts::Two($task, $path, &Fun{f: $f1}, &Fun{f: $f2})
    };
    ($task:expr, $path:expr, $f:expr) => {
        Parts::One($task, $path, &Fun{f: $f})
    }
}

fn main() {
    let days = [
        day!("Day 01", "day1", &day1::solve_part_1, &day1::solve_part_2),
        day!("Day 02", "day2", &day2::solve_part_1, &day2::solve_part_2),
        day!("Day 03", "day3", &day3::solve_part_1, &day3::solve_part_2),
        day!("Day 04", "day4", &day4::solve_part_1, &day4::solve_part_2),
        day!("Day 05", "day5", &day5::solve_part_1, &day5::solve_part_2),
        day!("Day 06", "day6", &day6::solve_part_1, &day6::solve_part_2),
        day!("Day 07", "day7", &day7::solve_part_1, &day7::solve_part_2),
        day!("Day 08", "day8", &day8::solve_part_1, &day8::solve_part_2)
    ];
	
    let run_count = 5;

    let days: Vec<_> = days.into_iter()
        .flat_map(|part| match part {
            Parts::One(task, path, p) => 
                vec!((task, "part 1", path, p)),
            Parts::Two(task, path, p1, p2) =>
                vec!((task, "part 1", path, p1), (task, "part 2", path, p2))
        })
        .collect();

    let mut times = vec!();
    for (task, part, path, f) in days.into_iter() {
        println!("Benchmarking {task} {part}");

        let file_path = format!("../{path}/input/input.txt");
        let content = fs::read_to_string(file_path.clone())
            .expect(format!("file {} should be present", file_path).as_str());

        let mut curr_times = vec!();
        for _ in 0..run_count {
            curr_times.push(f.benchmark(&content.clone()).as_secs_f64() * 1000.0);
        }
        let avg = curr_times.iter().sum::<f64>() / curr_times.len() as f64;
        times.push((format!("{task} {part}"), curr_times, avg));
    }

    let w = run_count;
    println!("| Task | {} Average |", (0..w).map(|i| format!(" Run {} |", i + 1)).collect::<String>());
    println!("|----------|{}----------|", (0..w).map(|_| format!("----------|")).collect::<String>());

    for (task, times, avg) in times {
        println!("| {task} | {} {avg:.3}ms |", times.iter().map(|t| format!(" {t:.3}ms |")).collect::<String>());
    }
}

trait Benchmark {
    fn benchmark(&self, input: &String) -> Duration;
}

struct Fun<'a, T> {
    f: &'a dyn Fn(&String) -> T
}

impl<'a, T> Benchmark for Fun<'a, T> {
    fn benchmark(&self, input: &String) -> Duration {
        let inst = Instant::now();
        black_box((self.f)(black_box(input)));
        return inst.elapsed();
    }
}

#[derive(Clone)]
enum Parts<'a> {
    One(&'a str, &'a str, &'a dyn Benchmark),
    Two(&'a str, &'a str, &'a dyn Benchmark, &'a dyn Benchmark)
}