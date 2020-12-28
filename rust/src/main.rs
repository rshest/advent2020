#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::time;

mod common;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

const SOLUTIONS: [Option<fn() -> ()>; 25] = [
    Some(day01::solution),
    Some(day02::solution),
    Some(day03::solution),
    Some(day04::solution),
    Some(day05::solution),
    Some(day06::solution),
    Some(day07::solution),
    Some(day08::solution),
    Some(day09::solution),
    Some(day10::solution),
    Some(day11::solution),
    Some(day12::solution),
    Some(day13::solution),
    Some(day14::solution),
    Some(day15::solution),
    Some(day16::solution),
    Some(day17::solution),
    Some(day18::solution),
    Some(day19::solution),
    Some(day20::solution),
    Some(day21::solution),
    Some(day22::solution),
    Some(day23::solution),
    Some(day24::solution),
    Some(day25::solution),
];

fn main() {
    let mut total_elapsed: time::Duration = time::Duration::new(0, 0);
    let mut total_problems: usize = 0;
    let mut timings: Vec<u128> = Vec::new();
    for (i, solution) in SOLUTIONS.iter().enumerate() {
        println!("--- Day{:02} ---", i + 1);
        match solution {
            None => println!("<TODO>\n"),
            Some(f) => {
                let now = time::Instant::now();
                f();
                let elapsed = now.elapsed();
                timings.push(elapsed.as_micros());
                println!("Elapsed: {:.2?}\n", elapsed);
                total_elapsed += elapsed;
                total_problems += 1;
            }
        }
    }
    println!(
        "Total problems: {}, elapsed: {:.2?}",
        total_problems, total_elapsed
    );
    println!("\nProblem timings (mus): {:?}", timings);
}
