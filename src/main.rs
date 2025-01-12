mod aoc;
mod days;

use crate::aoc::Statistics;
use crate::days::*;
use clap::Parser;
use std::error::Error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The day of the puzzle to run (mutually exclusive with --perf)
    #[arg(group = "mode")]
    day: Option<usize>,
    /// Run performance tests (mutually exclusive with day)
    #[arg(long, group = "mode")]
    perf: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut solutions = solutions();
    let args = Args::parse();

    match args.day {
        None => {
            let mut data = Vec::new();
            for i in 0..solutions.len() {
                let datum = solutions[i].run(i + 1)?;
                if !args.perf {
                    println!("{datum}");
                    println!("");
                }
                data.push(datum);
            }
            if args.perf {
                let stats = Statistics::calc(&data);
                println!("{stats}");
                println!("");
                print_slowest(&stats, 5);
                println!("");
            }
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
