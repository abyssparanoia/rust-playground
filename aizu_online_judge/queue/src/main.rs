use std::collections::VecDeque;
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

#[derive(Copy)]
struct Process {
    name: String,
    time: i32,
}

impl Process {

    fn new(name:String,time:i32) -> Process {
        Process{name,time}
    }

    fn forward(&mut self, quantum: i32) {
        self.time - quantum;
    }

    fn is_finish(self) -> bool {
        self.time <= 0
    }
}

struct WaitingProcessQueue {
    queue: VecDeque<Process>,
    quantum: i32,
    total_time: i32,
}

impl WaitingProcessQueue {

    fn new(input_queue: VecDeque<Process>, quantum:i32) -> WaitingProcessQueue {
        WaitingProcessQueue{queue:input_queue, quantum:quantum, total_time:0}
    }

    fn forward(&mut self) {
        while self.queue.len() > 0 {
            self.next();
        }
    }

    fn next(&mut self) -> bool {
        let front_process = self.queue.pop_front();
        match front_process {
            Some(mut process) => {
                process.forward(self.quantum);
                self.increment_total_time(process.time);
                if process.is_finish() {
                    println!("{} {}", process.name, process.time);
                } else {
                    self.queue.push_back(process);
                }
                true
            }
            None => false,
        }
    }

    fn increment_total_time(&mut self, added_time: i32) {
        self.total_time += self.quantum
    }
}

fn main() {
    let n: i32 = read();
    let q: i32 = read();

    let process_queue: VecDeque<Process> = VecDeque::new();

    for _ in 0..n {
        let name: String = read();
        let time: i32 = read();
        process_queue.push_back(Process::new(name, time));
    }

    WaitingProcessQueue::new(process_queue, q).forward();
}
