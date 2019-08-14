use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

#[derive(Debug, Copy, Clone, Eq, Hash)]
struct Job {
    _id: usize,
    day: i32,
    reward: i32,
}

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        self.reward.cmp(&other.reward)
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.reward == other.reward
    }
}

#![feature(drain_filter)]
fn main() {
    let n: usize = read();
    let m: usize = read();

    let mut jobs: Vec<Job> = (0..n)
        .map(|id| {
            let day: i32 = read();
            let reward: i32 = read();

            Job {
                _id: id,
                day: day,
                reward: reward,
            }
        })
        .collect();

    let mut max: i32 = 0;

    for j in 1..m + 1 {
        // println!("{:?}", jobs);
        let mut bh = BinaryHeap::new();
        for i in 0..jobs.len() {
            if (j as i32) >= jobs[i].day {
                bh.push(jobs[i]);
            }
        }
        let max_job_option = bh.pop();
        if max_job_option != None {
            let max_job = max_job_option.unwrap();
            max += max_job.reward;
            println!("{}", 0 == max_job._id);
            println!("{}", 1 == max_job._id);
            jobs.drain_filter(|&mut job| job._id == max_job._id);
        }
    }

    println!("{}", max)
}
