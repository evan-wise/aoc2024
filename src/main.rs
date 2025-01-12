mod aoc;
mod days;
use aoc::Statistics;

use crate::days::*;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut solutions = solutions();
    match parse_args()? {
        None => {
            let mut data = Vec::new();
            for i in 0..solutions.len() {
                let datum = solutions[i].run(i + 1)?;
                println!("{datum}");
                println!("");
                data.push(datum);
            }
            let stats = Statistics::calc(&data);
            println!("{stats}");
            println!("");
            print_slowest(&stats, 5);
            println!("");
        }
        Some(solution_num) => {
            if solution_num < 1 || solution_num > solutions.len() {
                return Err("argument out of range".into());
            }
            println!("{}", solutions[solution_num - 1].run(solution_num)?);
        }
    }
    Ok(())
}

fn parse_args() -> Result<Option<usize>, Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() > 1 {
        return Err("too many arguments".into());
    }

    if args.len() == 0 {
        return Ok(None);
    }
    Ok(Some(args[0].parse::<usize>()?))
}

fn print_slowest(stats: &Statistics, n: usize) {
    let mut slowest = stats.complete.clone();
    slowest.sort_by(|a, b| (b.parse_time + b.solve_time).cmp(&(a.parse_time + a.solve_time)));
    let avg = stats.avg / 1000.0;
    let stddev = stats.stddev / 1000.0;
    println!("~- SLOWEST -~");
    for line in slowest[0..n].iter().map(|s| {
        let time = (s.parse_time + s.solve_time).as_micros() as f64 / 1000.0;
        format!(
            "DAY {:0>2}: {:.3}ms ({:+.2}σ)",
            s.num,
            time,
            (time - avg) / stddev
        )
    }) {
        println!("{line}");
    }
}
